

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

#ifndef __taskschd_h__
#define __taskschd_h__

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

#ifndef __ITaskFolderCollection_FWD_DEFINED__
#define __ITaskFolderCollection_FWD_DEFINED__
typedef interface ITaskFolderCollection ITaskFolderCollection;

#endif 	/* __ITaskFolderCollection_FWD_DEFINED__ */


#ifndef __ITaskService_FWD_DEFINED__
#define __ITaskService_FWD_DEFINED__
typedef interface ITaskService ITaskService;

#endif 	/* __ITaskService_FWD_DEFINED__ */


#ifndef __ITaskHandler_FWD_DEFINED__
#define __ITaskHandler_FWD_DEFINED__
typedef interface ITaskHandler ITaskHandler;

#endif 	/* __ITaskHandler_FWD_DEFINED__ */


#ifndef __ITaskHandlerStatus_FWD_DEFINED__
#define __ITaskHandlerStatus_FWD_DEFINED__
typedef interface ITaskHandlerStatus ITaskHandlerStatus;

#endif 	/* __ITaskHandlerStatus_FWD_DEFINED__ */


#ifndef __ITaskVariables_FWD_DEFINED__
#define __ITaskVariables_FWD_DEFINED__
typedef interface ITaskVariables ITaskVariables;

#endif 	/* __ITaskVariables_FWD_DEFINED__ */


#ifndef __ITaskNamedValuePair_FWD_DEFINED__
#define __ITaskNamedValuePair_FWD_DEFINED__
typedef interface ITaskNamedValuePair ITaskNamedValuePair;

#endif 	/* __ITaskNamedValuePair_FWD_DEFINED__ */


#ifndef __ITaskNamedValueCollection_FWD_DEFINED__
#define __ITaskNamedValueCollection_FWD_DEFINED__
typedef interface ITaskNamedValueCollection ITaskNamedValueCollection;

#endif 	/* __ITaskNamedValueCollection_FWD_DEFINED__ */


#ifndef __IRunningTask_FWD_DEFINED__
#define __IRunningTask_FWD_DEFINED__
typedef interface IRunningTask IRunningTask;

#endif 	/* __IRunningTask_FWD_DEFINED__ */


#ifndef __IRunningTaskCollection_FWD_DEFINED__
#define __IRunningTaskCollection_FWD_DEFINED__
typedef interface IRunningTaskCollection IRunningTaskCollection;

#endif 	/* __IRunningTaskCollection_FWD_DEFINED__ */


#ifndef __IRegisteredTask_FWD_DEFINED__
#define __IRegisteredTask_FWD_DEFINED__
typedef interface IRegisteredTask IRegisteredTask;

#endif 	/* __IRegisteredTask_FWD_DEFINED__ */


#ifndef __ITrigger_FWD_DEFINED__
#define __ITrigger_FWD_DEFINED__
typedef interface ITrigger ITrigger;

#endif 	/* __ITrigger_FWD_DEFINED__ */


#ifndef __IIdleTrigger_FWD_DEFINED__
#define __IIdleTrigger_FWD_DEFINED__
typedef interface IIdleTrigger IIdleTrigger;

#endif 	/* __IIdleTrigger_FWD_DEFINED__ */


#ifndef __ILogonTrigger_FWD_DEFINED__
#define __ILogonTrigger_FWD_DEFINED__
typedef interface ILogonTrigger ILogonTrigger;

#endif 	/* __ILogonTrigger_FWD_DEFINED__ */


#ifndef __ISessionStateChangeTrigger_FWD_DEFINED__
#define __ISessionStateChangeTrigger_FWD_DEFINED__
typedef interface ISessionStateChangeTrigger ISessionStateChangeTrigger;

#endif 	/* __ISessionStateChangeTrigger_FWD_DEFINED__ */


#ifndef __IEventTrigger_FWD_DEFINED__
#define __IEventTrigger_FWD_DEFINED__
typedef interface IEventTrigger IEventTrigger;

#endif 	/* __IEventTrigger_FWD_DEFINED__ */


#ifndef __ITimeTrigger_FWD_DEFINED__
#define __ITimeTrigger_FWD_DEFINED__
typedef interface ITimeTrigger ITimeTrigger;

#endif 	/* __ITimeTrigger_FWD_DEFINED__ */


#ifndef __IDailyTrigger_FWD_DEFINED__
#define __IDailyTrigger_FWD_DEFINED__
typedef interface IDailyTrigger IDailyTrigger;

#endif 	/* __IDailyTrigger_FWD_DEFINED__ */


#ifndef __IWeeklyTrigger_FWD_DEFINED__
#define __IWeeklyTrigger_FWD_DEFINED__
typedef interface IWeeklyTrigger IWeeklyTrigger;

#endif 	/* __IWeeklyTrigger_FWD_DEFINED__ */


#ifndef __IMonthlyTrigger_FWD_DEFINED__
#define __IMonthlyTrigger_FWD_DEFINED__
typedef interface IMonthlyTrigger IMonthlyTrigger;

#endif 	/* __IMonthlyTrigger_FWD_DEFINED__ */


#ifndef __IMonthlyDOWTrigger_FWD_DEFINED__
#define __IMonthlyDOWTrigger_FWD_DEFINED__
typedef interface IMonthlyDOWTrigger IMonthlyDOWTrigger;

#endif 	/* __IMonthlyDOWTrigger_FWD_DEFINED__ */


#ifndef __IBootTrigger_FWD_DEFINED__
#define __IBootTrigger_FWD_DEFINED__
typedef interface IBootTrigger IBootTrigger;

#endif 	/* __IBootTrigger_FWD_DEFINED__ */


#ifndef __IRegistrationTrigger_FWD_DEFINED__
#define __IRegistrationTrigger_FWD_DEFINED__
typedef interface IRegistrationTrigger IRegistrationTrigger;

#endif 	/* __IRegistrationTrigger_FWD_DEFINED__ */


#ifndef __IAction_FWD_DEFINED__
#define __IAction_FWD_DEFINED__
typedef interface IAction IAction;

#endif 	/* __IAction_FWD_DEFINED__ */


#ifndef __IExecAction_FWD_DEFINED__
#define __IExecAction_FWD_DEFINED__
typedef interface IExecAction IExecAction;

#endif 	/* __IExecAction_FWD_DEFINED__ */


#ifndef __IExecAction2_FWD_DEFINED__
#define __IExecAction2_FWD_DEFINED__
typedef interface IExecAction2 IExecAction2;

#endif 	/* __IExecAction2_FWD_DEFINED__ */


#ifndef __IShowMessageAction_FWD_DEFINED__
#define __IShowMessageAction_FWD_DEFINED__
typedef interface IShowMessageAction IShowMessageAction;

#endif 	/* __IShowMessageAction_FWD_DEFINED__ */


#ifndef __IComHandlerAction_FWD_DEFINED__
#define __IComHandlerAction_FWD_DEFINED__
typedef interface IComHandlerAction IComHandlerAction;

#endif 	/* __IComHandlerAction_FWD_DEFINED__ */


#ifndef __IEmailAction_FWD_DEFINED__
#define __IEmailAction_FWD_DEFINED__
typedef interface IEmailAction IEmailAction;

#endif 	/* __IEmailAction_FWD_DEFINED__ */


#ifndef __ITriggerCollection_FWD_DEFINED__
#define __ITriggerCollection_FWD_DEFINED__
typedef interface ITriggerCollection ITriggerCollection;

#endif 	/* __ITriggerCollection_FWD_DEFINED__ */


#ifndef __IActionCollection_FWD_DEFINED__
#define __IActionCollection_FWD_DEFINED__
typedef interface IActionCollection IActionCollection;

#endif 	/* __IActionCollection_FWD_DEFINED__ */


#ifndef __IPrincipal_FWD_DEFINED__
#define __IPrincipal_FWD_DEFINED__
typedef interface IPrincipal IPrincipal;

#endif 	/* __IPrincipal_FWD_DEFINED__ */


#ifndef __IPrincipal2_FWD_DEFINED__
#define __IPrincipal2_FWD_DEFINED__
typedef interface IPrincipal2 IPrincipal2;

#endif 	/* __IPrincipal2_FWD_DEFINED__ */


#ifndef __IRegistrationInfo_FWD_DEFINED__
#define __IRegistrationInfo_FWD_DEFINED__
typedef interface IRegistrationInfo IRegistrationInfo;

#endif 	/* __IRegistrationInfo_FWD_DEFINED__ */


#ifndef __ITaskDefinition_FWD_DEFINED__
#define __ITaskDefinition_FWD_DEFINED__
typedef interface ITaskDefinition ITaskDefinition;

#endif 	/* __ITaskDefinition_FWD_DEFINED__ */


#ifndef __ITaskSettings_FWD_DEFINED__
#define __ITaskSettings_FWD_DEFINED__
typedef interface ITaskSettings ITaskSettings;

#endif 	/* __ITaskSettings_FWD_DEFINED__ */


#ifndef __ITaskSettings2_FWD_DEFINED__
#define __ITaskSettings2_FWD_DEFINED__
typedef interface ITaskSettings2 ITaskSettings2;

#endif 	/* __ITaskSettings2_FWD_DEFINED__ */


#ifndef __ITaskSettings3_FWD_DEFINED__
#define __ITaskSettings3_FWD_DEFINED__
typedef interface ITaskSettings3 ITaskSettings3;

#endif 	/* __ITaskSettings3_FWD_DEFINED__ */


#ifndef __IMaintenanceSettings_FWD_DEFINED__
#define __IMaintenanceSettings_FWD_DEFINED__
typedef interface IMaintenanceSettings IMaintenanceSettings;

#endif 	/* __IMaintenanceSettings_FWD_DEFINED__ */


#ifndef __TaskScheduler_FWD_DEFINED__
#define __TaskScheduler_FWD_DEFINED__

#ifdef __cplusplus
typedef class TaskScheduler TaskScheduler;
#else
typedef struct TaskScheduler TaskScheduler;
#endif /* __cplusplus */

#endif 	/* __TaskScheduler_FWD_DEFINED__ */


#ifndef __TaskHandlerPS_FWD_DEFINED__
#define __TaskHandlerPS_FWD_DEFINED__

#ifdef __cplusplus
typedef class TaskHandlerPS TaskHandlerPS;
#else
typedef struct TaskHandlerPS TaskHandlerPS;
#endif /* __cplusplus */

#endif 	/* __TaskHandlerPS_FWD_DEFINED__ */


#ifndef __TaskHandlerStatusPS_FWD_DEFINED__
#define __TaskHandlerStatusPS_FWD_DEFINED__

#ifdef __cplusplus
typedef class TaskHandlerStatusPS TaskHandlerStatusPS;
#else
typedef struct TaskHandlerStatusPS TaskHandlerStatusPS;
#endif /* __cplusplus */

#endif 	/* __TaskHandlerStatusPS_FWD_DEFINED__ */


#ifndef __ITaskNamedValuePair_FWD_DEFINED__
#define __ITaskNamedValuePair_FWD_DEFINED__
typedef interface ITaskNamedValuePair ITaskNamedValuePair;

#endif 	/* __ITaskNamedValuePair_FWD_DEFINED__ */


#ifndef __ITaskNamedValueCollection_FWD_DEFINED__
#define __ITaskNamedValueCollection_FWD_DEFINED__
typedef interface ITaskNamedValueCollection ITaskNamedValueCollection;

#endif 	/* __ITaskNamedValueCollection_FWD_DEFINED__ */


#ifndef __IRunningTask_FWD_DEFINED__
#define __IRunningTask_FWD_DEFINED__
typedef interface IRunningTask IRunningTask;

#endif 	/* __IRunningTask_FWD_DEFINED__ */


#ifndef __IRunningTaskCollection_FWD_DEFINED__
#define __IRunningTaskCollection_FWD_DEFINED__
typedef interface IRunningTaskCollection IRunningTaskCollection;

#endif 	/* __IRunningTaskCollection_FWD_DEFINED__ */


#ifndef __IRegisteredTask_FWD_DEFINED__
#define __IRegisteredTask_FWD_DEFINED__
typedef interface IRegisteredTask IRegisteredTask;

#endif 	/* __IRegisteredTask_FWD_DEFINED__ */


#ifndef __IRegisteredTaskCollection_FWD_DEFINED__
#define __IRegisteredTaskCollection_FWD_DEFINED__
typedef interface IRegisteredTaskCollection IRegisteredTaskCollection;

#endif 	/* __IRegisteredTaskCollection_FWD_DEFINED__ */


#ifndef __ITaskFolder_FWD_DEFINED__
#define __ITaskFolder_FWD_DEFINED__
typedef interface ITaskFolder ITaskFolder;

#endif 	/* __ITaskFolder_FWD_DEFINED__ */


#ifndef __ITaskFolderCollection_FWD_DEFINED__
#define __ITaskFolderCollection_FWD_DEFINED__
typedef interface ITaskFolderCollection ITaskFolderCollection;

#endif 	/* __ITaskFolderCollection_FWD_DEFINED__ */


#ifndef __ITaskService_FWD_DEFINED__
#define __ITaskService_FWD_DEFINED__
typedef interface ITaskService ITaskService;

#endif 	/* __ITaskService_FWD_DEFINED__ */


#ifndef __IIdleSettings_FWD_DEFINED__
#define __IIdleSettings_FWD_DEFINED__
typedef interface IIdleSettings IIdleSettings;

#endif 	/* __IIdleSettings_FWD_DEFINED__ */


#ifndef __INetworkSettings_FWD_DEFINED__
#define __INetworkSettings_FWD_DEFINED__
typedef interface INetworkSettings INetworkSettings;

#endif 	/* __INetworkSettings_FWD_DEFINED__ */


#ifndef __IRepetitionPattern_FWD_DEFINED__
#define __IRepetitionPattern_FWD_DEFINED__
typedef interface IRepetitionPattern IRepetitionPattern;

#endif 	/* __IRepetitionPattern_FWD_DEFINED__ */


#ifndef __ITrigger_FWD_DEFINED__
#define __ITrigger_FWD_DEFINED__
typedef interface ITrigger ITrigger;

#endif 	/* __ITrigger_FWD_DEFINED__ */


#ifndef __IEventTrigger_FWD_DEFINED__
#define __IEventTrigger_FWD_DEFINED__
typedef interface IEventTrigger IEventTrigger;

#endif 	/* __IEventTrigger_FWD_DEFINED__ */


#ifndef __ITimeTrigger_FWD_DEFINED__
#define __ITimeTrigger_FWD_DEFINED__
typedef interface ITimeTrigger ITimeTrigger;

#endif 	/* __ITimeTrigger_FWD_DEFINED__ */


#ifndef __IDailyTrigger_FWD_DEFINED__
#define __IDailyTrigger_FWD_DEFINED__
typedef interface IDailyTrigger IDailyTrigger;

#endif 	/* __IDailyTrigger_FWD_DEFINED__ */


#ifndef __IWeeklyTrigger_FWD_DEFINED__
#define __IWeeklyTrigger_FWD_DEFINED__
typedef interface IWeeklyTrigger IWeeklyTrigger;

#endif 	/* __IWeeklyTrigger_FWD_DEFINED__ */


#ifndef __IMonthlyTrigger_FWD_DEFINED__
#define __IMonthlyTrigger_FWD_DEFINED__
typedef interface IMonthlyTrigger IMonthlyTrigger;

#endif 	/* __IMonthlyTrigger_FWD_DEFINED__ */


#ifndef __IMonthlyDOWTrigger_FWD_DEFINED__
#define __IMonthlyDOWTrigger_FWD_DEFINED__
typedef interface IMonthlyDOWTrigger IMonthlyDOWTrigger;

#endif 	/* __IMonthlyDOWTrigger_FWD_DEFINED__ */


#ifndef __IIdleTrigger_FWD_DEFINED__
#define __IIdleTrigger_FWD_DEFINED__
typedef interface IIdleTrigger IIdleTrigger;

#endif 	/* __IIdleTrigger_FWD_DEFINED__ */


#ifndef __ILogonTrigger_FWD_DEFINED__
#define __ILogonTrigger_FWD_DEFINED__
typedef interface ILogonTrigger ILogonTrigger;

#endif 	/* __ILogonTrigger_FWD_DEFINED__ */


#ifndef __ISessionStateChangeTrigger_FWD_DEFINED__
#define __ISessionStateChangeTrigger_FWD_DEFINED__
typedef interface ISessionStateChangeTrigger ISessionStateChangeTrigger;

#endif 	/* __ISessionStateChangeTrigger_FWD_DEFINED__ */


#ifndef __IBootTrigger_FWD_DEFINED__
#define __IBootTrigger_FWD_DEFINED__
typedef interface IBootTrigger IBootTrigger;

#endif 	/* __IBootTrigger_FWD_DEFINED__ */


#ifndef __IRegistrationTrigger_FWD_DEFINED__
#define __IRegistrationTrigger_FWD_DEFINED__
typedef interface IRegistrationTrigger IRegistrationTrigger;

#endif 	/* __IRegistrationTrigger_FWD_DEFINED__ */


#ifndef __IAction_FWD_DEFINED__
#define __IAction_FWD_DEFINED__
typedef interface IAction IAction;

#endif 	/* __IAction_FWD_DEFINED__ */


#ifndef __IExecAction_FWD_DEFINED__
#define __IExecAction_FWD_DEFINED__
typedef interface IExecAction IExecAction;

#endif 	/* __IExecAction_FWD_DEFINED__ */


#ifndef __IExecAction2_FWD_DEFINED__
#define __IExecAction2_FWD_DEFINED__
typedef interface IExecAction2 IExecAction2;

#endif 	/* __IExecAction2_FWD_DEFINED__ */


#ifndef __IShowMessageAction_FWD_DEFINED__
#define __IShowMessageAction_FWD_DEFINED__
typedef interface IShowMessageAction IShowMessageAction;

#endif 	/* __IShowMessageAction_FWD_DEFINED__ */


#ifndef __IComHandlerAction_FWD_DEFINED__
#define __IComHandlerAction_FWD_DEFINED__
typedef interface IComHandlerAction IComHandlerAction;

#endif 	/* __IComHandlerAction_FWD_DEFINED__ */


#ifndef __IEmailAction_FWD_DEFINED__
#define __IEmailAction_FWD_DEFINED__
typedef interface IEmailAction IEmailAction;

#endif 	/* __IEmailAction_FWD_DEFINED__ */


#ifndef __ITriggerCollection_FWD_DEFINED__
#define __ITriggerCollection_FWD_DEFINED__
typedef interface ITriggerCollection ITriggerCollection;

#endif 	/* __ITriggerCollection_FWD_DEFINED__ */


#ifndef __IActionCollection_FWD_DEFINED__
#define __IActionCollection_FWD_DEFINED__
typedef interface IActionCollection IActionCollection;

#endif 	/* __IActionCollection_FWD_DEFINED__ */


#ifndef __IPrincipal_FWD_DEFINED__
#define __IPrincipal_FWD_DEFINED__
typedef interface IPrincipal IPrincipal;

#endif 	/* __IPrincipal_FWD_DEFINED__ */


#ifndef __IPrincipal2_FWD_DEFINED__
#define __IPrincipal2_FWD_DEFINED__
typedef interface IPrincipal2 IPrincipal2;

#endif 	/* __IPrincipal2_FWD_DEFINED__ */


#ifndef __IRegistrationInfo_FWD_DEFINED__
#define __IRegistrationInfo_FWD_DEFINED__
typedef interface IRegistrationInfo IRegistrationInfo;

#endif 	/* __IRegistrationInfo_FWD_DEFINED__ */


#ifndef __ITaskSettings_FWD_DEFINED__
#define __ITaskSettings_FWD_DEFINED__
typedef interface ITaskSettings ITaskSettings;

#endif 	/* __ITaskSettings_FWD_DEFINED__ */


#ifndef __ITaskSettings2_FWD_DEFINED__
#define __ITaskSettings2_FWD_DEFINED__
typedef interface ITaskSettings2 ITaskSettings2;

#endif 	/* __ITaskSettings2_FWD_DEFINED__ */


#ifndef __ITaskSettings3_FWD_DEFINED__
#define __ITaskSettings3_FWD_DEFINED__
typedef interface ITaskSettings3 ITaskSettings3;

#endif 	/* __ITaskSettings3_FWD_DEFINED__ */


#ifndef __IMaintenanceSettings_FWD_DEFINED__
#define __IMaintenanceSettings_FWD_DEFINED__
typedef interface IMaintenanceSettings IMaintenanceSettings;

#endif 	/* __IMaintenanceSettings_FWD_DEFINED__ */


#ifndef __ITaskDefinition_FWD_DEFINED__
#define __ITaskDefinition_FWD_DEFINED__
typedef interface ITaskDefinition ITaskDefinition;

#endif 	/* __ITaskDefinition_FWD_DEFINED__ */


#ifndef __ITaskVariables_FWD_DEFINED__
#define __ITaskVariables_FWD_DEFINED__
typedef interface ITaskVariables ITaskVariables;

#endif 	/* __ITaskVariables_FWD_DEFINED__ */


#ifndef __ITaskHandlerStatus_FWD_DEFINED__
#define __ITaskHandlerStatus_FWD_DEFINED__
typedef interface ITaskHandlerStatus ITaskHandlerStatus;

#endif 	/* __ITaskHandlerStatus_FWD_DEFINED__ */


#ifndef __ITaskHandler_FWD_DEFINED__
#define __ITaskHandler_FWD_DEFINED__
typedef interface ITaskHandler ITaskHandler;

#endif 	/* __ITaskHandler_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_taskschd_0000_0000 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (C) 2004-2008 Microsoft Corporation.  All rights reserved.     */
/*                                                                             */
/*    IDL source for Task Scheduler V2 COM API.                                */
/*                                                                             */
/*******************************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_taskschd_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_taskschd_0000_0000_v0_0_s_ifspec;


#ifndef __TaskScheduler_LIBRARY_DEFINED__
#define __TaskScheduler_LIBRARY_DEFINED__

/* library TaskScheduler */
/* [version][uuid] */ 







































typedef 
enum _TASK_RUN_FLAGS
    {
        TASK_RUN_NO_FLAGS	= 0,
        TASK_RUN_AS_SELF	= 0x1,
        TASK_RUN_IGNORE_CONSTRAINTS	= 0x2,
        TASK_RUN_USE_SESSION_ID	= 0x4,
        TASK_RUN_USER_SID	= 0x8
    } 	TASK_RUN_FLAGS;

typedef 
enum _TASK_ENUM_FLAGS
    {
        TASK_ENUM_HIDDEN	= 0x1
    } 	TASK_ENUM_FLAGS;

typedef 
enum _TASK_LOGON_TYPE
    {
        TASK_LOGON_NONE	= 0,
        TASK_LOGON_PASSWORD	= 1,
        TASK_LOGON_S4U	= 2,
        TASK_LOGON_INTERACTIVE_TOKEN	= 3,
        TASK_LOGON_GROUP	= 4,
        TASK_LOGON_SERVICE_ACCOUNT	= 5,
        TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD	= 6
    } 	TASK_LOGON_TYPE;

typedef 
enum _TASK_RUNLEVEL
    {
        TASK_RUNLEVEL_LUA	= 0,
        TASK_RUNLEVEL_HIGHEST	= 1
    } 	TASK_RUNLEVEL_TYPE;

typedef 
enum _TASK_PROCESSTOKENSID
    {
        TASK_PROCESSTOKENSID_NONE	= 0,
        TASK_PROCESSTOKENSID_UNRESTRICTED	= 1,
        TASK_PROCESSTOKENSID_DEFAULT	= 2
    } 	TASK_PROCESSTOKENSID_TYPE;

typedef 
enum _TASK_STATE
    {
        TASK_STATE_UNKNOWN	= 0,
        TASK_STATE_DISABLED	= 1,
        TASK_STATE_QUEUED	= 2,
        TASK_STATE_READY	= 3,
        TASK_STATE_RUNNING	= 4
    } 	TASK_STATE;

typedef 
enum _TASK_CREATION
    {
        TASK_VALIDATE_ONLY	= 0x1,
        TASK_CREATE	= 0x2,
        TASK_UPDATE	= 0x4,
        TASK_CREATE_OR_UPDATE	= ( TASK_CREATE | TASK_UPDATE ) ,
        TASK_DISABLE	= 0x8,
        TASK_DONT_ADD_PRINCIPAL_ACE	= 0x10,
        TASK_IGNORE_REGISTRATION_TRIGGERS	= 0x20
    } 	TASK_CREATION;

typedef 
enum _TASK_TRIGGER_TYPE2
    {
        TASK_TRIGGER_EVENT	= 0,
        TASK_TRIGGER_TIME	= 1,
        TASK_TRIGGER_DAILY	= 2,
        TASK_TRIGGER_WEEKLY	= 3,
        TASK_TRIGGER_MONTHLY	= 4,
        TASK_TRIGGER_MONTHLYDOW	= 5,
        TASK_TRIGGER_IDLE	= 6,
        TASK_TRIGGER_REGISTRATION	= 7,
        TASK_TRIGGER_BOOT	= 8,
        TASK_TRIGGER_LOGON	= 9,
        TASK_TRIGGER_SESSION_STATE_CHANGE	= 11,
        TASK_TRIGGER_CUSTOM_TRIGGER_01	= 12
    } 	TASK_TRIGGER_TYPE2;

typedef 
enum _TASK_SESSION_STATE_CHANGE_TYPE
    {
        TASK_CONSOLE_CONNECT	= 1,
        TASK_CONSOLE_DISCONNECT	= 2,
        TASK_REMOTE_CONNECT	= 3,
        TASK_REMOTE_DISCONNECT	= 4,
        TASK_SESSION_LOCK	= 7,
        TASK_SESSION_UNLOCK	= 8
    } 	TASK_SESSION_STATE_CHANGE_TYPE;

typedef 
enum _TASK_ACTION_TYPE
    {
        TASK_ACTION_EXEC	= 0,
        TASK_ACTION_COM_HANDLER	= 5,
        TASK_ACTION_SEND_EMAIL	= 6,
        TASK_ACTION_SHOW_MESSAGE	= 7
    } 	TASK_ACTION_TYPE;

typedef 
enum _TASK_INSTANCES_POLICY
    {
        TASK_INSTANCES_PARALLEL	= 0,
        TASK_INSTANCES_QUEUE	= 1,
        TASK_INSTANCES_IGNORE_NEW	= 2,
        TASK_INSTANCES_STOP_EXISTING	= 3
    } 	TASK_INSTANCES_POLICY;

typedef 
enum _TASK_COMPATIBILITY
    {
        TASK_COMPATIBILITY_AT	= 0,
        TASK_COMPATIBILITY_V1	= 1,
        TASK_COMPATIBILITY_V2	= 2,
        TASK_COMPATIBILITY_V2_1	= 3,
        TASK_COMPATIBILITY_V2_2	= 4,
        TASK_COMPATIBILITY_V2_3	= 5,
        TASK_COMPATIBILITY_V2_4	= 6
    } 	TASK_COMPATIBILITY;


EXTERN_C const IID LIBID_TaskScheduler;

#ifndef __ITaskFolderCollection_INTERFACE_DEFINED__
#define __ITaskFolderCollection_INTERFACE_DEFINED__

/* interface ITaskFolderCollection */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITaskFolderCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79184a66-8664-423f-97f1-637356a5d812")
    ITaskFolderCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskFolderCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskFolderCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskFolderCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskFolderCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITaskFolderCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITaskFolderCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITaskFolderCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskFolderCollection * This,
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
        
        DECLSPEC_XFGVIRT(ITaskFolderCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITaskFolderCollection * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ITaskFolderCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITaskFolderCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder);
        
        DECLSPEC_XFGVIRT(ITaskFolderCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITaskFolderCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        END_INTERFACE
    } ITaskFolderCollectionVtbl;

    interface ITaskFolderCollection
    {
        CONST_VTBL struct ITaskFolderCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskFolderCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskFolderCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskFolderCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskFolderCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskFolderCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskFolderCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskFolderCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskFolderCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define ITaskFolderCollection_get_Item(This,index,ppFolder)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppFolder) ) 

#define ITaskFolderCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskFolderCollection_INTERFACE_DEFINED__ */


#ifndef __ITaskService_INTERFACE_DEFINED__
#define __ITaskService_INTERFACE_DEFINED__

/* interface ITaskService */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITaskService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2faba4c7-4da9-4013-9697-20cc3fd40f85")
    ITaskService : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFolder( 
            /* [unique][in] */ __RPC__in_opt BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRunningTasks( 
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTaskCollection **ppRunningTasks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NewTask( 
            /* [in] */ DWORD flags,
            /* [retval][out] */ __RPC__deref_out_opt ITaskDefinition **ppDefinition) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [optional][in] */ VARIANT serverName,
            /* [optional][in] */ VARIANT user,
            /* [optional][in] */ VARIANT domain,
            /* [optional][in] */ VARIANT password) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Connected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pConnected) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetServer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pServer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectedUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pUser) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectedDomain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pDomain) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HighestVersion( 
            /* [retval][out] */ __RPC__out DWORD *pVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskService * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITaskService * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITaskService * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITaskService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskService * This,
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
        
        DECLSPEC_XFGVIRT(ITaskService, GetFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFolder )( 
            __RPC__in ITaskService * This,
            /* [unique][in] */ __RPC__in_opt BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder);
        
        DECLSPEC_XFGVIRT(ITaskService, GetRunningTasks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRunningTasks )( 
            __RPC__in ITaskService * This,
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTaskCollection **ppRunningTasks);
        
        DECLSPEC_XFGVIRT(ITaskService, NewTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NewTask )( 
            __RPC__in ITaskService * This,
            /* [in] */ DWORD flags,
            /* [retval][out] */ __RPC__deref_out_opt ITaskDefinition **ppDefinition);
        
        DECLSPEC_XFGVIRT(ITaskService, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ITaskService * This,
            /* [optional][in] */ VARIANT serverName,
            /* [optional][in] */ VARIANT user,
            /* [optional][in] */ VARIANT domain,
            /* [optional][in] */ VARIANT password);
        
        DECLSPEC_XFGVIRT(ITaskService, get_Connected)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Connected )( 
            __RPC__in ITaskService * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pConnected);
        
        DECLSPEC_XFGVIRT(ITaskService, get_TargetServer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetServer )( 
            __RPC__in ITaskService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pServer);
        
        DECLSPEC_XFGVIRT(ITaskService, get_ConnectedUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectedUser )( 
            __RPC__in ITaskService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pUser);
        
        DECLSPEC_XFGVIRT(ITaskService, get_ConnectedDomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectedDomain )( 
            __RPC__in ITaskService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pDomain);
        
        DECLSPEC_XFGVIRT(ITaskService, get_HighestVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighestVersion )( 
            __RPC__in ITaskService * This,
            /* [retval][out] */ __RPC__out DWORD *pVersion);
        
        END_INTERFACE
    } ITaskServiceVtbl;

    interface ITaskService
    {
        CONST_VTBL struct ITaskServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskService_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskService_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskService_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskService_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskService_GetFolder(This,path,ppFolder)	\
    ( (This)->lpVtbl -> GetFolder(This,path,ppFolder) ) 

#define ITaskService_GetRunningTasks(This,flags,ppRunningTasks)	\
    ( (This)->lpVtbl -> GetRunningTasks(This,flags,ppRunningTasks) ) 

#define ITaskService_NewTask(This,flags,ppDefinition)	\
    ( (This)->lpVtbl -> NewTask(This,flags,ppDefinition) ) 

#define ITaskService_Connect(This,serverName,user,domain,password)	\
    ( (This)->lpVtbl -> Connect(This,serverName,user,domain,password) ) 

#define ITaskService_get_Connected(This,pConnected)	\
    ( (This)->lpVtbl -> get_Connected(This,pConnected) ) 

#define ITaskService_get_TargetServer(This,pServer)	\
    ( (This)->lpVtbl -> get_TargetServer(This,pServer) ) 

#define ITaskService_get_ConnectedUser(This,pUser)	\
    ( (This)->lpVtbl -> get_ConnectedUser(This,pUser) ) 

#define ITaskService_get_ConnectedDomain(This,pDomain)	\
    ( (This)->lpVtbl -> get_ConnectedDomain(This,pDomain) ) 

#define ITaskService_get_HighestVersion(This,pVersion)	\
    ( (This)->lpVtbl -> get_HighestVersion(This,pVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskService_INTERFACE_DEFINED__ */


#ifndef __ITaskHandler_INTERFACE_DEFINED__
#define __ITaskHandler_INTERFACE_DEFINED__

/* interface ITaskHandler */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITaskHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("839d7762-5121-4009-9234-4f0d19394f04")
    ITaskHandler : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ __RPC__in_opt IUnknown *pHandlerServices,
            /* [in] */ __RPC__in BSTR data) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Stop( 
            /* [out] */ __RPC__out HRESULT *pRetCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskHandler * This);
        
        DECLSPEC_XFGVIRT(ITaskHandler, Start)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in ITaskHandler * This,
            /* [in] */ __RPC__in_opt IUnknown *pHandlerServices,
            /* [in] */ __RPC__in BSTR data);
        
        DECLSPEC_XFGVIRT(ITaskHandler, Stop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in ITaskHandler * This,
            /* [out] */ __RPC__out HRESULT *pRetCode);
        
        DECLSPEC_XFGVIRT(ITaskHandler, Pause)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in ITaskHandler * This);
        
        DECLSPEC_XFGVIRT(ITaskHandler, Resume)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in ITaskHandler * This);
        
        END_INTERFACE
    } ITaskHandlerVtbl;

    interface ITaskHandler
    {
        CONST_VTBL struct ITaskHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskHandler_Start(This,pHandlerServices,data)	\
    ( (This)->lpVtbl -> Start(This,pHandlerServices,data) ) 

#define ITaskHandler_Stop(This,pRetCode)	\
    ( (This)->lpVtbl -> Stop(This,pRetCode) ) 

#define ITaskHandler_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define ITaskHandler_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskHandler_INTERFACE_DEFINED__ */


#ifndef __ITaskHandlerStatus_INTERFACE_DEFINED__
#define __ITaskHandlerStatus_INTERFACE_DEFINED__

/* interface ITaskHandlerStatus */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITaskHandlerStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eaec7a8f-27a0-4ddc-8675-14726a01a38a")
    ITaskHandlerStatus : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UpdateStatus( 
            /* [in] */ short percentComplete,
            /* [in] */ __RPC__in BSTR statusMessage) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TaskCompleted( 
            /* [in] */ HRESULT taskErrCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskHandlerStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskHandlerStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskHandlerStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskHandlerStatus * This);
        
        DECLSPEC_XFGVIRT(ITaskHandlerStatus, UpdateStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateStatus )( 
            __RPC__in ITaskHandlerStatus * This,
            /* [in] */ short percentComplete,
            /* [in] */ __RPC__in BSTR statusMessage);
        
        DECLSPEC_XFGVIRT(ITaskHandlerStatus, TaskCompleted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TaskCompleted )( 
            __RPC__in ITaskHandlerStatus * This,
            /* [in] */ HRESULT taskErrCode);
        
        END_INTERFACE
    } ITaskHandlerStatusVtbl;

    interface ITaskHandlerStatus
    {
        CONST_VTBL struct ITaskHandlerStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskHandlerStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskHandlerStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskHandlerStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskHandlerStatus_UpdateStatus(This,percentComplete,statusMessage)	\
    ( (This)->lpVtbl -> UpdateStatus(This,percentComplete,statusMessage) ) 

#define ITaskHandlerStatus_TaskCompleted(This,taskErrCode)	\
    ( (This)->lpVtbl -> TaskCompleted(This,taskErrCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskHandlerStatus_INTERFACE_DEFINED__ */


#ifndef __ITaskVariables_INTERFACE_DEFINED__
#define __ITaskVariables_INTERFACE_DEFINED__

/* interface ITaskVariables */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITaskVariables;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e4c9351-d966-4b8b-bb87-ceba68bb0107")
    ITaskVariables : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetInput( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pInput) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetOutput( 
            /* [in] */ __RPC__in BSTR input) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetContext( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskVariablesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskVariables * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskVariables * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskVariables * This);
        
        DECLSPEC_XFGVIRT(ITaskVariables, GetInput)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetInput )( 
            __RPC__in ITaskVariables * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pInput);
        
        DECLSPEC_XFGVIRT(ITaskVariables, SetOutput)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetOutput )( 
            __RPC__in ITaskVariables * This,
            /* [in] */ __RPC__in BSTR input);
        
        DECLSPEC_XFGVIRT(ITaskVariables, GetContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITaskVariables * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContext);
        
        END_INTERFACE
    } ITaskVariablesVtbl;

    interface ITaskVariables
    {
        CONST_VTBL struct ITaskVariablesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskVariables_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskVariables_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskVariables_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskVariables_GetInput(This,pInput)	\
    ( (This)->lpVtbl -> GetInput(This,pInput) ) 

#define ITaskVariables_SetOutput(This,input)	\
    ( (This)->lpVtbl -> SetOutput(This,input) ) 

#define ITaskVariables_GetContext(This,pContext)	\
    ( (This)->lpVtbl -> GetContext(This,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskVariables_INTERFACE_DEFINED__ */


#ifndef __ITaskNamedValuePair_INTERFACE_DEFINED__
#define __ITaskNamedValuePair_INTERFACE_DEFINED__

/* interface ITaskNamedValuePair */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskNamedValuePair;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39038068-2B46-4afd-8662-7BB6F868D221")
    ITaskNamedValuePair : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *pName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ BSTR name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ BSTR *pValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ BSTR value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskNamedValuePairVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskNamedValuePair * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskNamedValuePair * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskNamedValuePair * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskNamedValuePair * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskNamedValuePair * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskNamedValuePair * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskNamedValuePair * This,
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
        
        DECLSPEC_XFGVIRT(ITaskNamedValuePair, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ITaskNamedValuePair * This,
            /* [retval][out] */ BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITaskNamedValuePair, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            ITaskNamedValuePair * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(ITaskNamedValuePair, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            ITaskNamedValuePair * This,
            /* [retval][out] */ BSTR *pValue);
        
        DECLSPEC_XFGVIRT(ITaskNamedValuePair, put_Value)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            ITaskNamedValuePair * This,
            /* [in] */ BSTR value);
        
        END_INTERFACE
    } ITaskNamedValuePairVtbl;

    interface ITaskNamedValuePair
    {
        CONST_VTBL struct ITaskNamedValuePairVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskNamedValuePair_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskNamedValuePair_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskNamedValuePair_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskNamedValuePair_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskNamedValuePair_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskNamedValuePair_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskNamedValuePair_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskNamedValuePair_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITaskNamedValuePair_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define ITaskNamedValuePair_get_Value(This,pValue)	\
    ( (This)->lpVtbl -> get_Value(This,pValue) ) 

#define ITaskNamedValuePair_put_Value(This,value)	\
    ( (This)->lpVtbl -> put_Value(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskNamedValuePair_INTERFACE_DEFINED__ */


#ifndef __ITaskNamedValueCollection_INTERFACE_DEFINED__
#define __ITaskNamedValueCollection_INTERFACE_DEFINED__

/* interface ITaskNamedValueCollection */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskNamedValueCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B4EF826B-63C3-46e4-A504-EF69E4F7EA4D")
    ITaskNamedValueCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG index,
            /* [retval][out] */ ITaskNamedValuePair **ppPair) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **ppEnum) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ BSTR name,
            /* [in] */ BSTR value,
            /* [retval][out] */ ITaskNamedValuePair **ppPair) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ LONG index) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskNamedValueCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskNamedValueCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskNamedValueCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskNamedValueCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskNamedValueCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskNamedValueCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskNamedValueCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskNamedValueCollection * This,
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
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ITaskNamedValueCollection * This,
            /* [retval][out] */ long *pCount);
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            ITaskNamedValueCollection * This,
            /* [in] */ LONG index,
            /* [retval][out] */ ITaskNamedValuePair **ppPair);
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ITaskNamedValueCollection * This,
            /* [retval][out] */ IUnknown **ppEnum);
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, Create)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            ITaskNamedValueCollection * This,
            /* [in] */ BSTR name,
            /* [in] */ BSTR value,
            /* [retval][out] */ ITaskNamedValuePair **ppPair);
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ITaskNamedValueCollection * This,
            /* [in] */ LONG index);
        
        DECLSPEC_XFGVIRT(ITaskNamedValueCollection, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            ITaskNamedValueCollection * This);
        
        END_INTERFACE
    } ITaskNamedValueCollectionVtbl;

    interface ITaskNamedValueCollection
    {
        CONST_VTBL struct ITaskNamedValueCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskNamedValueCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskNamedValueCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskNamedValueCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskNamedValueCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskNamedValueCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskNamedValueCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskNamedValueCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskNamedValueCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define ITaskNamedValueCollection_get_Item(This,index,ppPair)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppPair) ) 

#define ITaskNamedValueCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define ITaskNamedValueCollection_Create(This,name,value,ppPair)	\
    ( (This)->lpVtbl -> Create(This,name,value,ppPair) ) 

#define ITaskNamedValueCollection_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#define ITaskNamedValueCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskNamedValueCollection_INTERFACE_DEFINED__ */


#ifndef __IRunningTask_INTERFACE_DEFINED__
#define __IRunningTask_INTERFACE_DEFINED__

/* interface IRunningTask */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IRunningTask;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("653758fb-7b9a-4f1e-a471-beeb8e9b834e")
    IRunningTask : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InstanceGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pGuid) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out TASK_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentAction( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnginePID( 
            /* [retval][out] */ __RPC__out DWORD *pPID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRunningTaskVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRunningTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRunningTask * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRunningTask * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRunningTask * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRunningTask * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRunningTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRunningTask * This,
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
        
        DECLSPEC_XFGVIRT(IRunningTask, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(IRunningTask, get_InstanceGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InstanceGuid )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pGuid);
        
        DECLSPEC_XFGVIRT(IRunningTask, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath);
        
        DECLSPEC_XFGVIRT(IRunningTask, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__out TASK_STATE *pState);
        
        DECLSPEC_XFGVIRT(IRunningTask, get_CurrentAction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentAction )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(IRunningTask, Stop)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IRunningTask * This);
        
        DECLSPEC_XFGVIRT(IRunningTask, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IRunningTask * This);
        
        DECLSPEC_XFGVIRT(IRunningTask, get_EnginePID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnginePID )( 
            __RPC__in IRunningTask * This,
            /* [retval][out] */ __RPC__out DWORD *pPID);
        
        END_INTERFACE
    } IRunningTaskVtbl;

    interface IRunningTask
    {
        CONST_VTBL struct IRunningTaskVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRunningTask_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRunningTask_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRunningTask_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRunningTask_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRunningTask_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRunningTask_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRunningTask_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRunningTask_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define IRunningTask_get_InstanceGuid(This,pGuid)	\
    ( (This)->lpVtbl -> get_InstanceGuid(This,pGuid) ) 

#define IRunningTask_get_Path(This,pPath)	\
    ( (This)->lpVtbl -> get_Path(This,pPath) ) 

#define IRunningTask_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define IRunningTask_get_CurrentAction(This,pName)	\
    ( (This)->lpVtbl -> get_CurrentAction(This,pName) ) 

#define IRunningTask_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IRunningTask_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IRunningTask_get_EnginePID(This,pPID)	\
    ( (This)->lpVtbl -> get_EnginePID(This,pPID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRunningTask_INTERFACE_DEFINED__ */


#ifndef __IRunningTaskCollection_INTERFACE_DEFINED__
#define __IRunningTaskCollection_INTERFACE_DEFINED__

/* interface IRunningTaskCollection */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IRunningTaskCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6a67614b-6828-4fec-aa54-6d52e8f1f2db")
    IRunningTaskCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRunningTaskCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRunningTaskCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRunningTaskCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRunningTaskCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRunningTaskCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRunningTaskCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRunningTaskCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRunningTaskCollection * This,
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
        
        DECLSPEC_XFGVIRT(IRunningTaskCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IRunningTaskCollection * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(IRunningTaskCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IRunningTaskCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask);
        
        DECLSPEC_XFGVIRT(IRunningTaskCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IRunningTaskCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        END_INTERFACE
    } IRunningTaskCollectionVtbl;

    interface IRunningTaskCollection
    {
        CONST_VTBL struct IRunningTaskCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRunningTaskCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRunningTaskCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRunningTaskCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRunningTaskCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRunningTaskCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRunningTaskCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRunningTaskCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRunningTaskCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IRunningTaskCollection_get_Item(This,index,ppRunningTask)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppRunningTask) ) 

#define IRunningTaskCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRunningTaskCollection_INTERFACE_DEFINED__ */


#ifndef __IRegisteredTask_INTERFACE_DEFINED__
#define __IRegisteredTask_INTERFACE_DEFINED__

/* interface IRegisteredTask */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IRegisteredTask;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c86f320-dee3-4dd1-b972-a303f26b061e")
    IRegisteredTask : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out TASK_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            VARIANT_BOOL enabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ VARIANT params,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RunEx( 
            /* [in] */ VARIANT params,
            /* [in] */ LONG flags,
            /* [in] */ LONG sessionID,
            /* [in] */ __RPC__in BSTR user,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInstances( 
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTaskCollection **ppRunningTasks) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastRunTime( 
            /* [retval][out] */ __RPC__out DATE *pLastRunTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastTaskResult( 
            /* [retval][out] */ __RPC__out LONG *pLastTaskResult) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfMissedRuns( 
            /* [retval][out] */ __RPC__out LONG *pNumberOfMissedRuns) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NextRunTime( 
            /* [retval][out] */ __RPC__out DATE *pNextRunTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Definition( 
            /* [retval][out] */ __RPC__deref_out_opt ITaskDefinition **ppDefinition) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Xml( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pXml) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityDescriptor( 
            /* [in] */ LONG securityInformation,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSddl) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityDescriptor( 
            /* [in] */ __RPC__in BSTR sddl,
            /* [in] */ LONG flags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Stop( 
            /* [in] */ LONG flags) = 0;
        
        virtual /* [helpstring][hidden][restricted] */ HRESULT STDMETHODCALLTYPE GetRunTimes( 
            /* [in] */ __RPC__in const LPSYSTEMTIME pstStart,
            /* [in] */ __RPC__in const LPSYSTEMTIME pstEnd,
            /* [out][in] */ __RPC__inout DWORD *pCount,
            /* [out] */ __RPC__deref_out_opt LPSYSTEMTIME *pRunTimes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegisteredTaskVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRegisteredTask * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRegisteredTask * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRegisteredTask * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRegisteredTask * This,
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
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out TASK_STATE *pState);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, put_Enabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IRegisteredTask * This,
            VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, Run)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ VARIANT params,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, RunEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RunEx )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ VARIANT params,
            /* [in] */ LONG flags,
            /* [in] */ LONG sessionID,
            /* [in] */ __RPC__in BSTR user,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTask **ppRunningTask);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, GetInstances)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInstances )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRunningTaskCollection **ppRunningTasks);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_LastRunTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastRunTime )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out DATE *pLastRunTime);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_LastTaskResult)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastTaskResult )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out LONG *pLastTaskResult);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_NumberOfMissedRuns)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfMissedRuns )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out LONG *pNumberOfMissedRuns);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_NextRunTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NextRunTime )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__out DATE *pNextRunTime);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_Definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Definition )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__deref_out_opt ITaskDefinition **ppDefinition);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, get_Xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IRegisteredTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pXml);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, GetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ LONG securityInformation,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSddl);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, SetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ __RPC__in BSTR sddl,
            /* [in] */ LONG flags);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, Stop)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ LONG flags);
        
        DECLSPEC_XFGVIRT(IRegisteredTask, GetRunTimes)
        /* [helpstring][hidden][restricted] */ HRESULT ( STDMETHODCALLTYPE *GetRunTimes )( 
            __RPC__in IRegisteredTask * This,
            /* [in] */ __RPC__in const LPSYSTEMTIME pstStart,
            /* [in] */ __RPC__in const LPSYSTEMTIME pstEnd,
            /* [out][in] */ __RPC__inout DWORD *pCount,
            /* [out] */ __RPC__deref_out_opt LPSYSTEMTIME *pRunTimes);
        
        END_INTERFACE
    } IRegisteredTaskVtbl;

    interface IRegisteredTask
    {
        CONST_VTBL struct IRegisteredTaskVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegisteredTask_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegisteredTask_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegisteredTask_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegisteredTask_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRegisteredTask_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRegisteredTask_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRegisteredTask_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRegisteredTask_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define IRegisteredTask_get_Path(This,pPath)	\
    ( (This)->lpVtbl -> get_Path(This,pPath) ) 

#define IRegisteredTask_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define IRegisteredTask_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IRegisteredTask_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define IRegisteredTask_Run(This,params,ppRunningTask)	\
    ( (This)->lpVtbl -> Run(This,params,ppRunningTask) ) 

#define IRegisteredTask_RunEx(This,params,flags,sessionID,user,ppRunningTask)	\
    ( (This)->lpVtbl -> RunEx(This,params,flags,sessionID,user,ppRunningTask) ) 

#define IRegisteredTask_GetInstances(This,flags,ppRunningTasks)	\
    ( (This)->lpVtbl -> GetInstances(This,flags,ppRunningTasks) ) 

#define IRegisteredTask_get_LastRunTime(This,pLastRunTime)	\
    ( (This)->lpVtbl -> get_LastRunTime(This,pLastRunTime) ) 

#define IRegisteredTask_get_LastTaskResult(This,pLastTaskResult)	\
    ( (This)->lpVtbl -> get_LastTaskResult(This,pLastTaskResult) ) 

#define IRegisteredTask_get_NumberOfMissedRuns(This,pNumberOfMissedRuns)	\
    ( (This)->lpVtbl -> get_NumberOfMissedRuns(This,pNumberOfMissedRuns) ) 

#define IRegisteredTask_get_NextRunTime(This,pNextRunTime)	\
    ( (This)->lpVtbl -> get_NextRunTime(This,pNextRunTime) ) 

#define IRegisteredTask_get_Definition(This,ppDefinition)	\
    ( (This)->lpVtbl -> get_Definition(This,ppDefinition) ) 

#define IRegisteredTask_get_Xml(This,pXml)	\
    ( (This)->lpVtbl -> get_Xml(This,pXml) ) 

#define IRegisteredTask_GetSecurityDescriptor(This,securityInformation,pSddl)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,securityInformation,pSddl) ) 

#define IRegisteredTask_SetSecurityDescriptor(This,sddl,flags)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,sddl,flags) ) 

#define IRegisteredTask_Stop(This,flags)	\
    ( (This)->lpVtbl -> Stop(This,flags) ) 

#define IRegisteredTask_GetRunTimes(This,pstStart,pstEnd,pCount,pRunTimes)	\
    ( (This)->lpVtbl -> GetRunTimes(This,pstStart,pstEnd,pCount,pRunTimes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegisteredTask_INTERFACE_DEFINED__ */


#ifndef __ITrigger_INTERFACE_DEFINED__
#define __ITrigger_INTERFACE_DEFINED__

/* interface ITrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09941815-ea89-4b5b-89e0-2a773801fac3")
    ITrigger : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ BSTR *pId) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Id( 
            /* [in] */ BSTR id) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Repetition( 
            /* [retval][out] */ IRepetitionPattern **ppRepeat) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Repetition( 
            /* [in] */ IRepetitionPattern *pRepeat) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ExecutionTimeLimit( 
            /* [retval][out] */ BSTR *pTimeLimit) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ExecutionTimeLimit( 
            /* [in] */ BSTR timelimit) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StartBoundary( 
            /* [retval][out] */ BSTR *pStart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StartBoundary( 
            /* [in] */ BSTR start) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_EndBoundary( 
            /* [retval][out] */ BSTR *pEnd) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_EndBoundary( 
            /* [in] */ BSTR end) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ VARIANT_BOOL *pEnabled) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            ITrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            ITrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            ITrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            ITrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            ITrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ITrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ITrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            ITrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            ITrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            ITrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            ITrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ITrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ITrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        END_INTERFACE
    } ITriggerVtbl;

    interface ITrigger
    {
        CONST_VTBL struct ITriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ITrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define ITrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define ITrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define ITrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define ITrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define ITrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define ITrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define ITrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define ITrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define ITrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define ITrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ITrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITrigger_INTERFACE_DEFINED__ */


#ifndef __IIdleTrigger_INTERFACE_DEFINED__
#define __IIdleTrigger_INTERFACE_DEFINED__

/* interface IIdleTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IIdleTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d537d2b0-9fb3-4d34-9739-1ff5ce7b1ef3")
    IIdleTrigger : public ITrigger
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IIdleTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIdleTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIdleTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIdleTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IIdleTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IIdleTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IIdleTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIdleTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IIdleTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IIdleTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IIdleTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IIdleTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IIdleTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IIdleTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IIdleTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IIdleTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IIdleTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IIdleTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IIdleTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IIdleTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IIdleTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        END_INTERFACE
    } IIdleTriggerVtbl;

    interface IIdleTrigger
    {
        CONST_VTBL struct IIdleTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdleTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdleTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdleTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdleTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIdleTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIdleTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIdleTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIdleTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IIdleTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IIdleTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IIdleTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IIdleTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IIdleTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IIdleTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IIdleTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IIdleTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IIdleTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IIdleTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IIdleTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IIdleTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdleTrigger_INTERFACE_DEFINED__ */


#ifndef __ILogonTrigger_INTERFACE_DEFINED__
#define __ILogonTrigger_INTERFACE_DEFINED__

/* interface ILogonTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ILogonTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72DADE38-FAE4-4b3e-BAF4-5D009AF02B1C")
    ILogonTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Delay( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Delay( 
            /* [in] */ BSTR delay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UserId( 
            /* [retval][out] */ BSTR *pUser) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UserId( 
            /* [in] */ BSTR user) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILogonTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ILogonTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ILogonTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ILogonTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ILogonTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ILogonTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ILogonTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILogonTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            ILogonTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            ILogonTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            ILogonTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            ILogonTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ILogonTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            ILogonTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            ILogonTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ILogonTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ILogonTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(ILogonTrigger, get_Delay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Delay )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(ILogonTrigger, put_Delay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Delay )( 
            ILogonTrigger * This,
            /* [in] */ BSTR delay);
        
        DECLSPEC_XFGVIRT(ILogonTrigger, get_UserId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserId )( 
            ILogonTrigger * This,
            /* [retval][out] */ BSTR *pUser);
        
        DECLSPEC_XFGVIRT(ILogonTrigger, put_UserId)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserId )( 
            ILogonTrigger * This,
            /* [in] */ BSTR user);
        
        END_INTERFACE
    } ILogonTriggerVtbl;

    interface ILogonTrigger
    {
        CONST_VTBL struct ILogonTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILogonTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILogonTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILogonTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILogonTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILogonTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILogonTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILogonTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILogonTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ILogonTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define ILogonTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define ILogonTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define ILogonTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define ILogonTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define ILogonTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define ILogonTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define ILogonTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define ILogonTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define ILogonTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define ILogonTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ILogonTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define ILogonTrigger_get_Delay(This,pDelay)	\
    ( (This)->lpVtbl -> get_Delay(This,pDelay) ) 

#define ILogonTrigger_put_Delay(This,delay)	\
    ( (This)->lpVtbl -> put_Delay(This,delay) ) 

#define ILogonTrigger_get_UserId(This,pUser)	\
    ( (This)->lpVtbl -> get_UserId(This,pUser) ) 

#define ILogonTrigger_put_UserId(This,user)	\
    ( (This)->lpVtbl -> put_UserId(This,user) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILogonTrigger_INTERFACE_DEFINED__ */


#ifndef __ISessionStateChangeTrigger_INTERFACE_DEFINED__
#define __ISessionStateChangeTrigger_INTERFACE_DEFINED__

/* interface ISessionStateChangeTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISessionStateChangeTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("754DA71B-4385-4475-9DD9-598294FA3641")
    ISessionStateChangeTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Delay( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Delay( 
            /* [in] */ BSTR delay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UserId( 
            /* [retval][out] */ BSTR *pUser) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UserId( 
            /* [in] */ BSTR user) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StateChange( 
            /* [retval][out] */ TASK_SESSION_STATE_CHANGE_TYPE *pType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StateChange( 
            /* [in] */ TASK_SESSION_STATE_CHANGE_TYPE type) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISessionStateChangeTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISessionStateChangeTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISessionStateChangeTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISessionStateChangeTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISessionStateChangeTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, get_Delay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Delay )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, put_Delay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Delay )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR delay);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, get_UserId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserId )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ BSTR *pUser);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, put_UserId)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserId )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ BSTR user);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, get_StateChange)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StateChange )( 
            ISessionStateChangeTrigger * This,
            /* [retval][out] */ TASK_SESSION_STATE_CHANGE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(ISessionStateChangeTrigger, put_StateChange)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StateChange )( 
            ISessionStateChangeTrigger * This,
            /* [in] */ TASK_SESSION_STATE_CHANGE_TYPE type);
        
        END_INTERFACE
    } ISessionStateChangeTriggerVtbl;

    interface ISessionStateChangeTrigger
    {
        CONST_VTBL struct ISessionStateChangeTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISessionStateChangeTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISessionStateChangeTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISessionStateChangeTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISessionStateChangeTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISessionStateChangeTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISessionStateChangeTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISessionStateChangeTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISessionStateChangeTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ISessionStateChangeTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define ISessionStateChangeTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define ISessionStateChangeTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define ISessionStateChangeTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define ISessionStateChangeTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define ISessionStateChangeTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define ISessionStateChangeTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define ISessionStateChangeTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define ISessionStateChangeTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define ISessionStateChangeTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define ISessionStateChangeTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ISessionStateChangeTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define ISessionStateChangeTrigger_get_Delay(This,pDelay)	\
    ( (This)->lpVtbl -> get_Delay(This,pDelay) ) 

#define ISessionStateChangeTrigger_put_Delay(This,delay)	\
    ( (This)->lpVtbl -> put_Delay(This,delay) ) 

#define ISessionStateChangeTrigger_get_UserId(This,pUser)	\
    ( (This)->lpVtbl -> get_UserId(This,pUser) ) 

#define ISessionStateChangeTrigger_put_UserId(This,user)	\
    ( (This)->lpVtbl -> put_UserId(This,user) ) 

#define ISessionStateChangeTrigger_get_StateChange(This,pType)	\
    ( (This)->lpVtbl -> get_StateChange(This,pType) ) 

#define ISessionStateChangeTrigger_put_StateChange(This,type)	\
    ( (This)->lpVtbl -> put_StateChange(This,type) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISessionStateChangeTrigger_INTERFACE_DEFINED__ */


#ifndef __IEventTrigger_INTERFACE_DEFINED__
#define __IEventTrigger_INTERFACE_DEFINED__

/* interface IEventTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IEventTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d45b0167-9653-4eef-b94f-0732ca7af251")
    IEventTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Subscription( 
            /* [retval][out] */ BSTR *pQuery) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Subscription( 
            /* [in] */ BSTR query) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Delay( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Delay( 
            /* [in] */ BSTR delay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ValueQueries( 
            /* [retval][out] */ ITaskNamedValueCollection **ppNamedXPaths) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ValueQueries( 
            /* [in] */ ITaskNamedValueCollection *pNamedXPaths) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEventTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEventTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEventTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEventTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEventTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEventTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IEventTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IEventTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IEventTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IEventTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IEventTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IEventTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IEventTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IEventTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IEventTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IEventTrigger, get_Subscription)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Subscription )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pQuery);
        
        DECLSPEC_XFGVIRT(IEventTrigger, put_Subscription)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Subscription )( 
            IEventTrigger * This,
            /* [in] */ BSTR query);
        
        DECLSPEC_XFGVIRT(IEventTrigger, get_Delay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Delay )( 
            IEventTrigger * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(IEventTrigger, put_Delay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Delay )( 
            IEventTrigger * This,
            /* [in] */ BSTR delay);
        
        DECLSPEC_XFGVIRT(IEventTrigger, get_ValueQueries)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ValueQueries )( 
            IEventTrigger * This,
            /* [retval][out] */ ITaskNamedValueCollection **ppNamedXPaths);
        
        DECLSPEC_XFGVIRT(IEventTrigger, put_ValueQueries)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ValueQueries )( 
            IEventTrigger * This,
            /* [in] */ ITaskNamedValueCollection *pNamedXPaths);
        
        END_INTERFACE
    } IEventTriggerVtbl;

    interface IEventTrigger
    {
        CONST_VTBL struct IEventTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IEventTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IEventTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IEventTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IEventTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IEventTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IEventTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IEventTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IEventTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IEventTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IEventTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IEventTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IEventTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IEventTrigger_get_Subscription(This,pQuery)	\
    ( (This)->lpVtbl -> get_Subscription(This,pQuery) ) 

#define IEventTrigger_put_Subscription(This,query)	\
    ( (This)->lpVtbl -> put_Subscription(This,query) ) 

#define IEventTrigger_get_Delay(This,pDelay)	\
    ( (This)->lpVtbl -> get_Delay(This,pDelay) ) 

#define IEventTrigger_put_Delay(This,delay)	\
    ( (This)->lpVtbl -> put_Delay(This,delay) ) 

#define IEventTrigger_get_ValueQueries(This,ppNamedXPaths)	\
    ( (This)->lpVtbl -> get_ValueQueries(This,ppNamedXPaths) ) 

#define IEventTrigger_put_ValueQueries(This,pNamedXPaths)	\
    ( (This)->lpVtbl -> put_ValueQueries(This,pNamedXPaths) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventTrigger_INTERFACE_DEFINED__ */


#ifndef __ITimeTrigger_INTERFACE_DEFINED__
#define __ITimeTrigger_INTERFACE_DEFINED__

/* interface ITimeTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITimeTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b45747e0-eba7-4276-9f29-85c5bb300006")
    ITimeTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RandomDelay( 
            /* [retval][out] */ BSTR *pRandomDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RandomDelay( 
            /* [in] */ BSTR randomDelay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimeTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITimeTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITimeTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITimeTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITimeTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITimeTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITimeTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITimeTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            ITimeTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            ITimeTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            ITimeTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            ITimeTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            ITimeTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ITimeTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ITimeTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            ITimeTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            ITimeTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            ITimeTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            ITimeTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ITimeTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ITimeTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(ITimeTrigger, get_RandomDelay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RandomDelay )( 
            ITimeTrigger * This,
            /* [retval][out] */ BSTR *pRandomDelay);
        
        DECLSPEC_XFGVIRT(ITimeTrigger, put_RandomDelay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RandomDelay )( 
            ITimeTrigger * This,
            /* [in] */ BSTR randomDelay);
        
        END_INTERFACE
    } ITimeTriggerVtbl;

    interface ITimeTrigger
    {
        CONST_VTBL struct ITimeTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimeTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimeTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimeTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimeTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITimeTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITimeTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITimeTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITimeTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ITimeTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define ITimeTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define ITimeTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define ITimeTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define ITimeTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define ITimeTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define ITimeTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define ITimeTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define ITimeTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define ITimeTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define ITimeTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ITimeTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define ITimeTrigger_get_RandomDelay(This,pRandomDelay)	\
    ( (This)->lpVtbl -> get_RandomDelay(This,pRandomDelay) ) 

#define ITimeTrigger_put_RandomDelay(This,randomDelay)	\
    ( (This)->lpVtbl -> put_RandomDelay(This,randomDelay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimeTrigger_INTERFACE_DEFINED__ */


#ifndef __IDailyTrigger_INTERFACE_DEFINED__
#define __IDailyTrigger_INTERFACE_DEFINED__

/* interface IDailyTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IDailyTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("126c5cd8-b288-41d5-8dbf-e491446adc5c")
    IDailyTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DaysInterval( 
            /* [retval][out] */ short *pDays) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DaysInterval( 
            /* [in] */ short days) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RandomDelay( 
            /* [retval][out] */ BSTR *pRandomDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RandomDelay( 
            /* [in] */ BSTR randomDelay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDailyTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDailyTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDailyTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDailyTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IDailyTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IDailyTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IDailyTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDailyTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IDailyTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IDailyTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IDailyTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IDailyTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IDailyTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IDailyTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IDailyTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IDailyTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IDailyTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IDailyTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IDailyTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IDailyTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IDailyTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IDailyTrigger, get_DaysInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DaysInterval )( 
            IDailyTrigger * This,
            /* [retval][out] */ short *pDays);
        
        DECLSPEC_XFGVIRT(IDailyTrigger, put_DaysInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DaysInterval )( 
            IDailyTrigger * This,
            /* [in] */ short days);
        
        DECLSPEC_XFGVIRT(IDailyTrigger, get_RandomDelay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RandomDelay )( 
            IDailyTrigger * This,
            /* [retval][out] */ BSTR *pRandomDelay);
        
        DECLSPEC_XFGVIRT(IDailyTrigger, put_RandomDelay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RandomDelay )( 
            IDailyTrigger * This,
            /* [in] */ BSTR randomDelay);
        
        END_INTERFACE
    } IDailyTriggerVtbl;

    interface IDailyTrigger
    {
        CONST_VTBL struct IDailyTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDailyTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDailyTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDailyTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDailyTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDailyTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDailyTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDailyTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDailyTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IDailyTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IDailyTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IDailyTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IDailyTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IDailyTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IDailyTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IDailyTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IDailyTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IDailyTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IDailyTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IDailyTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IDailyTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IDailyTrigger_get_DaysInterval(This,pDays)	\
    ( (This)->lpVtbl -> get_DaysInterval(This,pDays) ) 

#define IDailyTrigger_put_DaysInterval(This,days)	\
    ( (This)->lpVtbl -> put_DaysInterval(This,days) ) 

#define IDailyTrigger_get_RandomDelay(This,pRandomDelay)	\
    ( (This)->lpVtbl -> get_RandomDelay(This,pRandomDelay) ) 

#define IDailyTrigger_put_RandomDelay(This,randomDelay)	\
    ( (This)->lpVtbl -> put_RandomDelay(This,randomDelay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDailyTrigger_INTERFACE_DEFINED__ */


#ifndef __IWeeklyTrigger_INTERFACE_DEFINED__
#define __IWeeklyTrigger_INTERFACE_DEFINED__

/* interface IWeeklyTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IWeeklyTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5038fc98-82ff-436d-8728-a512a57c9dc1")
    IWeeklyTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DaysOfWeek( 
            /* [retval][out] */ short *pDays) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DaysOfWeek( 
            /* [in] */ short days) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WeeksInterval( 
            /* [retval][out] */ short *pWeeks) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_WeeksInterval( 
            /* [in] */ short weeks) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RandomDelay( 
            /* [retval][out] */ BSTR *pRandomDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RandomDelay( 
            /* [in] */ BSTR randomDelay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWeeklyTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWeeklyTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWeeklyTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWeeklyTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWeeklyTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWeeklyTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWeeklyTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWeeklyTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IWeeklyTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IWeeklyTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IWeeklyTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IWeeklyTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IWeeklyTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IWeeklyTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, get_DaysOfWeek)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DaysOfWeek )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ short *pDays);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, put_DaysOfWeek)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DaysOfWeek )( 
            IWeeklyTrigger * This,
            /* [in] */ short days);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, get_WeeksInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WeeksInterval )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ short *pWeeks);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, put_WeeksInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WeeksInterval )( 
            IWeeklyTrigger * This,
            /* [in] */ short weeks);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, get_RandomDelay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RandomDelay )( 
            IWeeklyTrigger * This,
            /* [retval][out] */ BSTR *pRandomDelay);
        
        DECLSPEC_XFGVIRT(IWeeklyTrigger, put_RandomDelay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RandomDelay )( 
            IWeeklyTrigger * This,
            /* [in] */ BSTR randomDelay);
        
        END_INTERFACE
    } IWeeklyTriggerVtbl;

    interface IWeeklyTrigger
    {
        CONST_VTBL struct IWeeklyTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWeeklyTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWeeklyTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWeeklyTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWeeklyTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWeeklyTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWeeklyTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWeeklyTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWeeklyTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWeeklyTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IWeeklyTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IWeeklyTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IWeeklyTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IWeeklyTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IWeeklyTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IWeeklyTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IWeeklyTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IWeeklyTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IWeeklyTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IWeeklyTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IWeeklyTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IWeeklyTrigger_get_DaysOfWeek(This,pDays)	\
    ( (This)->lpVtbl -> get_DaysOfWeek(This,pDays) ) 

#define IWeeklyTrigger_put_DaysOfWeek(This,days)	\
    ( (This)->lpVtbl -> put_DaysOfWeek(This,days) ) 

#define IWeeklyTrigger_get_WeeksInterval(This,pWeeks)	\
    ( (This)->lpVtbl -> get_WeeksInterval(This,pWeeks) ) 

#define IWeeklyTrigger_put_WeeksInterval(This,weeks)	\
    ( (This)->lpVtbl -> put_WeeksInterval(This,weeks) ) 

#define IWeeklyTrigger_get_RandomDelay(This,pRandomDelay)	\
    ( (This)->lpVtbl -> get_RandomDelay(This,pRandomDelay) ) 

#define IWeeklyTrigger_put_RandomDelay(This,randomDelay)	\
    ( (This)->lpVtbl -> put_RandomDelay(This,randomDelay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWeeklyTrigger_INTERFACE_DEFINED__ */


#ifndef __IMonthlyTrigger_INTERFACE_DEFINED__
#define __IMonthlyTrigger_INTERFACE_DEFINED__

/* interface IMonthlyTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IMonthlyTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97c45ef1-6b02-4a1a-9c0e-1ebfba1500ac")
    IMonthlyTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DaysOfMonth( 
            /* [retval][out] */ long *pDays) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DaysOfMonth( 
            /* [in] */ long days) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MonthsOfYear( 
            /* [retval][out] */ short *pMonths) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MonthsOfYear( 
            /* [in] */ short months) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RunOnLastDayOfMonth( 
            /* [retval][out] */ VARIANT_BOOL *pLastDay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RunOnLastDayOfMonth( 
            /* [in] */ VARIANT_BOOL lastDay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RandomDelay( 
            /* [retval][out] */ BSTR *pRandomDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RandomDelay( 
            /* [in] */ BSTR randomDelay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMonthlyTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMonthlyTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMonthlyTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMonthlyTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMonthlyTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMonthlyTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMonthlyTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMonthlyTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IMonthlyTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IMonthlyTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IMonthlyTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IMonthlyTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IMonthlyTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IMonthlyTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, get_DaysOfMonth)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DaysOfMonth )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ long *pDays);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, put_DaysOfMonth)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DaysOfMonth )( 
            IMonthlyTrigger * This,
            /* [in] */ long days);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, get_MonthsOfYear)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonthsOfYear )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ short *pMonths);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, put_MonthsOfYear)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonthsOfYear )( 
            IMonthlyTrigger * This,
            /* [in] */ short months);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, get_RunOnLastDayOfMonth)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnLastDayOfMonth )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pLastDay);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, put_RunOnLastDayOfMonth)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnLastDayOfMonth )( 
            IMonthlyTrigger * This,
            /* [in] */ VARIANT_BOOL lastDay);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, get_RandomDelay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RandomDelay )( 
            IMonthlyTrigger * This,
            /* [retval][out] */ BSTR *pRandomDelay);
        
        DECLSPEC_XFGVIRT(IMonthlyTrigger, put_RandomDelay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RandomDelay )( 
            IMonthlyTrigger * This,
            /* [in] */ BSTR randomDelay);
        
        END_INTERFACE
    } IMonthlyTriggerVtbl;

    interface IMonthlyTrigger
    {
        CONST_VTBL struct IMonthlyTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMonthlyTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMonthlyTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMonthlyTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMonthlyTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMonthlyTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMonthlyTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMonthlyTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMonthlyTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IMonthlyTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IMonthlyTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IMonthlyTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IMonthlyTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IMonthlyTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IMonthlyTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IMonthlyTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IMonthlyTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IMonthlyTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IMonthlyTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IMonthlyTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IMonthlyTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IMonthlyTrigger_get_DaysOfMonth(This,pDays)	\
    ( (This)->lpVtbl -> get_DaysOfMonth(This,pDays) ) 

#define IMonthlyTrigger_put_DaysOfMonth(This,days)	\
    ( (This)->lpVtbl -> put_DaysOfMonth(This,days) ) 

#define IMonthlyTrigger_get_MonthsOfYear(This,pMonths)	\
    ( (This)->lpVtbl -> get_MonthsOfYear(This,pMonths) ) 

#define IMonthlyTrigger_put_MonthsOfYear(This,months)	\
    ( (This)->lpVtbl -> put_MonthsOfYear(This,months) ) 

#define IMonthlyTrigger_get_RunOnLastDayOfMonth(This,pLastDay)	\
    ( (This)->lpVtbl -> get_RunOnLastDayOfMonth(This,pLastDay) ) 

#define IMonthlyTrigger_put_RunOnLastDayOfMonth(This,lastDay)	\
    ( (This)->lpVtbl -> put_RunOnLastDayOfMonth(This,lastDay) ) 

#define IMonthlyTrigger_get_RandomDelay(This,pRandomDelay)	\
    ( (This)->lpVtbl -> get_RandomDelay(This,pRandomDelay) ) 

#define IMonthlyTrigger_put_RandomDelay(This,randomDelay)	\
    ( (This)->lpVtbl -> put_RandomDelay(This,randomDelay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMonthlyTrigger_INTERFACE_DEFINED__ */


#ifndef __IMonthlyDOWTrigger_INTERFACE_DEFINED__
#define __IMonthlyDOWTrigger_INTERFACE_DEFINED__

/* interface IMonthlyDOWTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IMonthlyDOWTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77d025a3-90fa-43aa-b52e-cda5499b946a")
    IMonthlyDOWTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DaysOfWeek( 
            /* [retval][out] */ short *pDays) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DaysOfWeek( 
            /* [in] */ short days) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WeeksOfMonth( 
            /* [retval][out] */ short *pWeeks) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_WeeksOfMonth( 
            /* [in] */ short weeks) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MonthsOfYear( 
            /* [retval][out] */ short *pMonths) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MonthsOfYear( 
            /* [in] */ short months) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RunOnLastWeekOfMonth( 
            /* [retval][out] */ VARIANT_BOOL *pLastWeek) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RunOnLastWeekOfMonth( 
            /* [in] */ VARIANT_BOOL lastWeek) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RandomDelay( 
            /* [retval][out] */ BSTR *pRandomDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RandomDelay( 
            /* [in] */ BSTR randomDelay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMonthlyDOWTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMonthlyDOWTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMonthlyDOWTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMonthlyDOWTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMonthlyDOWTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, get_DaysOfWeek)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DaysOfWeek )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ short *pDays);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, put_DaysOfWeek)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DaysOfWeek )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ short days);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, get_WeeksOfMonth)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WeeksOfMonth )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ short *pWeeks);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, put_WeeksOfMonth)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WeeksOfMonth )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ short weeks);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, get_MonthsOfYear)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonthsOfYear )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ short *pMonths);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, put_MonthsOfYear)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonthsOfYear )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ short months);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, get_RunOnLastWeekOfMonth)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnLastWeekOfMonth )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pLastWeek);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, put_RunOnLastWeekOfMonth)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnLastWeekOfMonth )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ VARIANT_BOOL lastWeek);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, get_RandomDelay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RandomDelay )( 
            IMonthlyDOWTrigger * This,
            /* [retval][out] */ BSTR *pRandomDelay);
        
        DECLSPEC_XFGVIRT(IMonthlyDOWTrigger, put_RandomDelay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RandomDelay )( 
            IMonthlyDOWTrigger * This,
            /* [in] */ BSTR randomDelay);
        
        END_INTERFACE
    } IMonthlyDOWTriggerVtbl;

    interface IMonthlyDOWTrigger
    {
        CONST_VTBL struct IMonthlyDOWTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMonthlyDOWTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMonthlyDOWTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMonthlyDOWTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMonthlyDOWTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMonthlyDOWTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMonthlyDOWTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMonthlyDOWTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMonthlyDOWTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IMonthlyDOWTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IMonthlyDOWTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IMonthlyDOWTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IMonthlyDOWTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IMonthlyDOWTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IMonthlyDOWTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IMonthlyDOWTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IMonthlyDOWTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IMonthlyDOWTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IMonthlyDOWTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IMonthlyDOWTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IMonthlyDOWTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IMonthlyDOWTrigger_get_DaysOfWeek(This,pDays)	\
    ( (This)->lpVtbl -> get_DaysOfWeek(This,pDays) ) 

#define IMonthlyDOWTrigger_put_DaysOfWeek(This,days)	\
    ( (This)->lpVtbl -> put_DaysOfWeek(This,days) ) 

#define IMonthlyDOWTrigger_get_WeeksOfMonth(This,pWeeks)	\
    ( (This)->lpVtbl -> get_WeeksOfMonth(This,pWeeks) ) 

#define IMonthlyDOWTrigger_put_WeeksOfMonth(This,weeks)	\
    ( (This)->lpVtbl -> put_WeeksOfMonth(This,weeks) ) 

#define IMonthlyDOWTrigger_get_MonthsOfYear(This,pMonths)	\
    ( (This)->lpVtbl -> get_MonthsOfYear(This,pMonths) ) 

#define IMonthlyDOWTrigger_put_MonthsOfYear(This,months)	\
    ( (This)->lpVtbl -> put_MonthsOfYear(This,months) ) 

#define IMonthlyDOWTrigger_get_RunOnLastWeekOfMonth(This,pLastWeek)	\
    ( (This)->lpVtbl -> get_RunOnLastWeekOfMonth(This,pLastWeek) ) 

#define IMonthlyDOWTrigger_put_RunOnLastWeekOfMonth(This,lastWeek)	\
    ( (This)->lpVtbl -> put_RunOnLastWeekOfMonth(This,lastWeek) ) 

#define IMonthlyDOWTrigger_get_RandomDelay(This,pRandomDelay)	\
    ( (This)->lpVtbl -> get_RandomDelay(This,pRandomDelay) ) 

#define IMonthlyDOWTrigger_put_RandomDelay(This,randomDelay)	\
    ( (This)->lpVtbl -> put_RandomDelay(This,randomDelay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMonthlyDOWTrigger_INTERFACE_DEFINED__ */


#ifndef __IBootTrigger_INTERFACE_DEFINED__
#define __IBootTrigger_INTERFACE_DEFINED__

/* interface IBootTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IBootTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A9C35DA-D357-41f4-BBC1-207AC1B1F3CB")
    IBootTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Delay( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Delay( 
            /* [in] */ BSTR delay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBootTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBootTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBootTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBootTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IBootTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IBootTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IBootTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBootTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IBootTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IBootTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IBootTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IBootTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IBootTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IBootTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IBootTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IBootTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IBootTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IBootTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IBootTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IBootTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IBootTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IBootTrigger, get_Delay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Delay )( 
            IBootTrigger * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(IBootTrigger, put_Delay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Delay )( 
            IBootTrigger * This,
            /* [in] */ BSTR delay);
        
        END_INTERFACE
    } IBootTriggerVtbl;

    interface IBootTrigger
    {
        CONST_VTBL struct IBootTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBootTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBootTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBootTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBootTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBootTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBootTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBootTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBootTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IBootTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IBootTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IBootTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IBootTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IBootTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IBootTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IBootTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IBootTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IBootTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IBootTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IBootTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IBootTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IBootTrigger_get_Delay(This,pDelay)	\
    ( (This)->lpVtbl -> get_Delay(This,pDelay) ) 

#define IBootTrigger_put_Delay(This,delay)	\
    ( (This)->lpVtbl -> put_Delay(This,delay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBootTrigger_INTERFACE_DEFINED__ */


#ifndef __IRegistrationTrigger_INTERFACE_DEFINED__
#define __IRegistrationTrigger_INTERFACE_DEFINED__

/* interface IRegistrationTrigger */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IRegistrationTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c8fec3a-c218-4e0c-b23d-629024db91a2")
    IRegistrationTrigger : public ITrigger
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Delay( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Delay( 
            /* [in] */ BSTR delay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegistrationTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRegistrationTrigger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRegistrationTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRegistrationTrigger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IRegistrationTrigger * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IRegistrationTrigger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IRegistrationTrigger * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRegistrationTrigger * This,
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
        
        DECLSPEC_XFGVIRT(ITrigger, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ TASK_TRIGGER_TYPE2 *pType);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IRegistrationTrigger * This,
            /* [in] */ BSTR id);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Repetition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Repetition )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ IRepetitionPattern **ppRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Repetition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Repetition )( 
            IRegistrationTrigger * This,
            /* [in] */ IRepetitionPattern *pRepeat);
        
        DECLSPEC_XFGVIRT(ITrigger, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ BSTR *pTimeLimit);
        
        DECLSPEC_XFGVIRT(ITrigger, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            IRegistrationTrigger * This,
            /* [in] */ BSTR timelimit);
        
        DECLSPEC_XFGVIRT(ITrigger, get_StartBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartBoundary )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ BSTR *pStart);
        
        DECLSPEC_XFGVIRT(ITrigger, put_StartBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartBoundary )( 
            IRegistrationTrigger * This,
            /* [in] */ BSTR start);
        
        DECLSPEC_XFGVIRT(ITrigger, get_EndBoundary)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EndBoundary )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ BSTR *pEnd);
        
        DECLSPEC_XFGVIRT(ITrigger, put_EndBoundary)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EndBoundary )( 
            IRegistrationTrigger * This,
            /* [in] */ BSTR end);
        
        DECLSPEC_XFGVIRT(ITrigger, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITrigger, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            IRegistrationTrigger * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IRegistrationTrigger, get_Delay)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Delay )( 
            IRegistrationTrigger * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(IRegistrationTrigger, put_Delay)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Delay )( 
            IRegistrationTrigger * This,
            /* [in] */ BSTR delay);
        
        END_INTERFACE
    } IRegistrationTriggerVtbl;

    interface IRegistrationTrigger
    {
        CONST_VTBL struct IRegistrationTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegistrationTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegistrationTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegistrationTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegistrationTrigger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRegistrationTrigger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRegistrationTrigger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRegistrationTrigger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRegistrationTrigger_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IRegistrationTrigger_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IRegistrationTrigger_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#define IRegistrationTrigger_get_Repetition(This,ppRepeat)	\
    ( (This)->lpVtbl -> get_Repetition(This,ppRepeat) ) 

#define IRegistrationTrigger_put_Repetition(This,pRepeat)	\
    ( (This)->lpVtbl -> put_Repetition(This,pRepeat) ) 

#define IRegistrationTrigger_get_ExecutionTimeLimit(This,pTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pTimeLimit) ) 

#define IRegistrationTrigger_put_ExecutionTimeLimit(This,timelimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,timelimit) ) 

#define IRegistrationTrigger_get_StartBoundary(This,pStart)	\
    ( (This)->lpVtbl -> get_StartBoundary(This,pStart) ) 

#define IRegistrationTrigger_put_StartBoundary(This,start)	\
    ( (This)->lpVtbl -> put_StartBoundary(This,start) ) 

#define IRegistrationTrigger_get_EndBoundary(This,pEnd)	\
    ( (This)->lpVtbl -> get_EndBoundary(This,pEnd) ) 

#define IRegistrationTrigger_put_EndBoundary(This,end)	\
    ( (This)->lpVtbl -> put_EndBoundary(This,end) ) 

#define IRegistrationTrigger_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define IRegistrationTrigger_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 


#define IRegistrationTrigger_get_Delay(This,pDelay)	\
    ( (This)->lpVtbl -> get_Delay(This,pDelay) ) 

#define IRegistrationTrigger_put_Delay(This,delay)	\
    ( (This)->lpVtbl -> put_Delay(This,delay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegistrationTrigger_INTERFACE_DEFINED__ */


#ifndef __IAction_INTERFACE_DEFINED__
#define __IAction_INTERFACE_DEFINED__

/* interface IAction */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAE54997-48B1-4cbe-9965-D6BE263EBEA4")
    IAction : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ BSTR *pId) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Id( 
            /* [in] */ BSTR Id) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ TASK_ACTION_TYPE *pType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAction * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IAction * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IAction * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IAction * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        END_INTERFACE
    } IActionVtbl;

    interface IAction
    {
        CONST_VTBL struct IActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAction_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IAction_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IAction_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAction_INTERFACE_DEFINED__ */


#ifndef __IExecAction_INTERFACE_DEFINED__
#define __IExecAction_INTERFACE_DEFINED__

/* interface IExecAction */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IExecAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c3d624d-fd6b-49a3-b9b7-09cb3cd3f047")
    IExecAction : public IAction
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ BSTR *pPath) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ BSTR path) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Arguments( 
            /* [retval][out] */ BSTR *pArgument) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Arguments( 
            /* [in] */ BSTR argument) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WorkingDirectory( 
            /* [retval][out] */ BSTR *pWorkingDirectory) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_WorkingDirectory( 
            /* [in] */ BSTR workingDirectory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExecActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExecAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExecAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExecAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IExecAction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IExecAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IExecAction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IExecAction * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IExecAction * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IExecAction * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IExecAction * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IExecAction, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            IExecAction * This,
            /* [retval][out] */ BSTR *pPath);
        
        DECLSPEC_XFGVIRT(IExecAction, put_Path)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            IExecAction * This,
            /* [in] */ BSTR path);
        
        DECLSPEC_XFGVIRT(IExecAction, get_Arguments)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Arguments )( 
            IExecAction * This,
            /* [retval][out] */ BSTR *pArgument);
        
        DECLSPEC_XFGVIRT(IExecAction, put_Arguments)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Arguments )( 
            IExecAction * This,
            /* [in] */ BSTR argument);
        
        DECLSPEC_XFGVIRT(IExecAction, get_WorkingDirectory)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            IExecAction * This,
            /* [retval][out] */ BSTR *pWorkingDirectory);
        
        DECLSPEC_XFGVIRT(IExecAction, put_WorkingDirectory)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            IExecAction * This,
            /* [in] */ BSTR workingDirectory);
        
        END_INTERFACE
    } IExecActionVtbl;

    interface IExecAction
    {
        CONST_VTBL struct IExecActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExecAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExecAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExecAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExecAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IExecAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IExecAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IExecAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IExecAction_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IExecAction_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IExecAction_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 


#define IExecAction_get_Path(This,pPath)	\
    ( (This)->lpVtbl -> get_Path(This,pPath) ) 

#define IExecAction_put_Path(This,path)	\
    ( (This)->lpVtbl -> put_Path(This,path) ) 

#define IExecAction_get_Arguments(This,pArgument)	\
    ( (This)->lpVtbl -> get_Arguments(This,pArgument) ) 

#define IExecAction_put_Arguments(This,argument)	\
    ( (This)->lpVtbl -> put_Arguments(This,argument) ) 

#define IExecAction_get_WorkingDirectory(This,pWorkingDirectory)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pWorkingDirectory) ) 

#define IExecAction_put_WorkingDirectory(This,workingDirectory)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,workingDirectory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExecAction_INTERFACE_DEFINED__ */


#ifndef __IExecAction2_INTERFACE_DEFINED__
#define __IExecAction2_INTERFACE_DEFINED__

/* interface IExecAction2 */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IExecAction2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2a82542-bda5-4e6b-9143-e2bf4F8987b6")
    IExecAction2 : public IExecAction
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HideAppWindow( 
            /* [retval][out] */ VARIANT_BOOL *pHideAppWindow) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_HideAppWindow( 
            /* [in] */ VARIANT_BOOL hideAppWindow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExecAction2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExecAction2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExecAction2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExecAction2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IExecAction2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IExecAction2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IExecAction2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IExecAction2 * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IExecAction2 * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IExecAction2 * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IExecAction2 * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IExecAction, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            IExecAction2 * This,
            /* [retval][out] */ BSTR *pPath);
        
        DECLSPEC_XFGVIRT(IExecAction, put_Path)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            IExecAction2 * This,
            /* [in] */ BSTR path);
        
        DECLSPEC_XFGVIRT(IExecAction, get_Arguments)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Arguments )( 
            IExecAction2 * This,
            /* [retval][out] */ BSTR *pArgument);
        
        DECLSPEC_XFGVIRT(IExecAction, put_Arguments)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Arguments )( 
            IExecAction2 * This,
            /* [in] */ BSTR argument);
        
        DECLSPEC_XFGVIRT(IExecAction, get_WorkingDirectory)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            IExecAction2 * This,
            /* [retval][out] */ BSTR *pWorkingDirectory);
        
        DECLSPEC_XFGVIRT(IExecAction, put_WorkingDirectory)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            IExecAction2 * This,
            /* [in] */ BSTR workingDirectory);
        
        DECLSPEC_XFGVIRT(IExecAction2, get_HideAppWindow)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HideAppWindow )( 
            IExecAction2 * This,
            /* [retval][out] */ VARIANT_BOOL *pHideAppWindow);
        
        DECLSPEC_XFGVIRT(IExecAction2, put_HideAppWindow)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_HideAppWindow )( 
            IExecAction2 * This,
            /* [in] */ VARIANT_BOOL hideAppWindow);
        
        END_INTERFACE
    } IExecAction2Vtbl;

    interface IExecAction2
    {
        CONST_VTBL struct IExecAction2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExecAction2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExecAction2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExecAction2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExecAction2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IExecAction2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IExecAction2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IExecAction2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IExecAction2_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IExecAction2_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IExecAction2_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 


#define IExecAction2_get_Path(This,pPath)	\
    ( (This)->lpVtbl -> get_Path(This,pPath) ) 

#define IExecAction2_put_Path(This,path)	\
    ( (This)->lpVtbl -> put_Path(This,path) ) 

#define IExecAction2_get_Arguments(This,pArgument)	\
    ( (This)->lpVtbl -> get_Arguments(This,pArgument) ) 

#define IExecAction2_put_Arguments(This,argument)	\
    ( (This)->lpVtbl -> put_Arguments(This,argument) ) 

#define IExecAction2_get_WorkingDirectory(This,pWorkingDirectory)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pWorkingDirectory) ) 

#define IExecAction2_put_WorkingDirectory(This,workingDirectory)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,workingDirectory) ) 


#define IExecAction2_get_HideAppWindow(This,pHideAppWindow)	\
    ( (This)->lpVtbl -> get_HideAppWindow(This,pHideAppWindow) ) 

#define IExecAction2_put_HideAppWindow(This,hideAppWindow)	\
    ( (This)->lpVtbl -> put_HideAppWindow(This,hideAppWindow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExecAction2_INTERFACE_DEFINED__ */


#ifndef __IShowMessageAction_INTERFACE_DEFINED__
#define __IShowMessageAction_INTERFACE_DEFINED__

/* interface IShowMessageAction */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IShowMessageAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("505E9E68-AF89-46b8-A30F-56162A83D537")
    IShowMessageAction : public IAction
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ BSTR *pTitle) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Title( 
            /* [in] */ BSTR title) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MessageBody( 
            /* [retval][out] */ BSTR *pMessageBody) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MessageBody( 
            /* [in] */ BSTR messageBody) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShowMessageActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IShowMessageAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IShowMessageAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IShowMessageAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IShowMessageAction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IShowMessageAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IShowMessageAction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShowMessageAction * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IShowMessageAction * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IShowMessageAction * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IShowMessageAction * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IShowMessageAction, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            IShowMessageAction * This,
            /* [retval][out] */ BSTR *pTitle);
        
        DECLSPEC_XFGVIRT(IShowMessageAction, put_Title)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Title )( 
            IShowMessageAction * This,
            /* [in] */ BSTR title);
        
        DECLSPEC_XFGVIRT(IShowMessageAction, get_MessageBody)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MessageBody )( 
            IShowMessageAction * This,
            /* [retval][out] */ BSTR *pMessageBody);
        
        DECLSPEC_XFGVIRT(IShowMessageAction, put_MessageBody)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MessageBody )( 
            IShowMessageAction * This,
            /* [in] */ BSTR messageBody);
        
        END_INTERFACE
    } IShowMessageActionVtbl;

    interface IShowMessageAction
    {
        CONST_VTBL struct IShowMessageActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShowMessageAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShowMessageAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShowMessageAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShowMessageAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShowMessageAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShowMessageAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShowMessageAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShowMessageAction_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IShowMessageAction_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IShowMessageAction_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 


#define IShowMessageAction_get_Title(This,pTitle)	\
    ( (This)->lpVtbl -> get_Title(This,pTitle) ) 

#define IShowMessageAction_put_Title(This,title)	\
    ( (This)->lpVtbl -> put_Title(This,title) ) 

#define IShowMessageAction_get_MessageBody(This,pMessageBody)	\
    ( (This)->lpVtbl -> get_MessageBody(This,pMessageBody) ) 

#define IShowMessageAction_put_MessageBody(This,messageBody)	\
    ( (This)->lpVtbl -> put_MessageBody(This,messageBody) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShowMessageAction_INTERFACE_DEFINED__ */


#ifndef __IComHandlerAction_INTERFACE_DEFINED__
#define __IComHandlerAction_INTERFACE_DEFINED__

/* interface IComHandlerAction */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IComHandlerAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D2FD252-75C5-4f66-90BA-2A7D8CC3039F")
    IComHandlerAction : public IAction
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ClassId( 
            /* [retval][out] */ BSTR *pClsid) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ClassId( 
            /* [in] */ BSTR clsid) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ BSTR *pData) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ BSTR data) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComHandlerActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IComHandlerAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IComHandlerAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IComHandlerAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IComHandlerAction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IComHandlerAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IComHandlerAction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComHandlerAction * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IComHandlerAction * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IComHandlerAction * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IComHandlerAction * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IComHandlerAction, get_ClassId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ClassId )( 
            IComHandlerAction * This,
            /* [retval][out] */ BSTR *pClsid);
        
        DECLSPEC_XFGVIRT(IComHandlerAction, put_ClassId)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ClassId )( 
            IComHandlerAction * This,
            /* [in] */ BSTR clsid);
        
        DECLSPEC_XFGVIRT(IComHandlerAction, get_Data)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            IComHandlerAction * This,
            /* [retval][out] */ BSTR *pData);
        
        DECLSPEC_XFGVIRT(IComHandlerAction, put_Data)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            IComHandlerAction * This,
            /* [in] */ BSTR data);
        
        END_INTERFACE
    } IComHandlerActionVtbl;

    interface IComHandlerAction
    {
        CONST_VTBL struct IComHandlerActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComHandlerAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComHandlerAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComHandlerAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComHandlerAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComHandlerAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComHandlerAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComHandlerAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComHandlerAction_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IComHandlerAction_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IComHandlerAction_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 


#define IComHandlerAction_get_ClassId(This,pClsid)	\
    ( (This)->lpVtbl -> get_ClassId(This,pClsid) ) 

#define IComHandlerAction_put_ClassId(This,clsid)	\
    ( (This)->lpVtbl -> put_ClassId(This,clsid) ) 

#define IComHandlerAction_get_Data(This,pData)	\
    ( (This)->lpVtbl -> get_Data(This,pData) ) 

#define IComHandlerAction_put_Data(This,data)	\
    ( (This)->lpVtbl -> put_Data(This,data) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComHandlerAction_INTERFACE_DEFINED__ */


#ifndef __IEmailAction_INTERFACE_DEFINED__
#define __IEmailAction_INTERFACE_DEFINED__

/* interface IEmailAction */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IEmailAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10F62C64-7E16-4314-A0C2-0C3683F99D40")
    IEmailAction : public IAction
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Server( 
            /* [retval][out] */ BSTR *pServer) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Server( 
            /* [in] */ BSTR server) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ BSTR *pSubject) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Subject( 
            /* [in] */ BSTR subject) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_To( 
            /* [retval][out] */ BSTR *pTo) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_To( 
            /* [in] */ BSTR to) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Cc( 
            /* [retval][out] */ BSTR *pCc) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Cc( 
            /* [in] */ BSTR cc) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Bcc( 
            /* [retval][out] */ BSTR *pBcc) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Bcc( 
            /* [in] */ BSTR bcc) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ReplyTo( 
            /* [retval][out] */ BSTR *pReplyTo) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReplyTo( 
            /* [in] */ BSTR replyTo) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_From( 
            /* [retval][out] */ BSTR *pFrom) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_From( 
            /* [in] */ BSTR from) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HeaderFields( 
            /* [retval][out] */ ITaskNamedValueCollection **ppHeaderFields) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_HeaderFields( 
            /* [in] */ ITaskNamedValueCollection *pHeaderFields) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ BSTR *pBody) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ BSTR body) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Attachments( 
            /* [retval][out] */ SAFEARRAY * *pAttachements) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Attachments( 
            /* [max_is][in] */ SAFEARRAY * pAttachements) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEmailActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEmailAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEmailAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEmailAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEmailAction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEmailAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEmailAction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEmailAction * This,
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
        
        DECLSPEC_XFGVIRT(IAction, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IAction, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IEmailAction * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IAction, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            IEmailAction * This,
            /* [retval][out] */ TASK_ACTION_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Server)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Server )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pServer);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Server)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Server )( 
            IEmailAction * This,
            /* [in] */ BSTR server);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Subject)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pSubject);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Subject)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Subject )( 
            IEmailAction * This,
            /* [in] */ BSTR subject);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_To)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_To )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pTo);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_To)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_To )( 
            IEmailAction * This,
            /* [in] */ BSTR to);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Cc)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Cc )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pCc);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Cc)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Cc )( 
            IEmailAction * This,
            /* [in] */ BSTR cc);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Bcc)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bcc )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pBcc);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Bcc)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bcc )( 
            IEmailAction * This,
            /* [in] */ BSTR bcc);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_ReplyTo)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReplyTo )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pReplyTo);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_ReplyTo)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReplyTo )( 
            IEmailAction * This,
            /* [in] */ BSTR replyTo);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_From)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_From )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pFrom);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_From)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_From )( 
            IEmailAction * This,
            /* [in] */ BSTR from);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_HeaderFields)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HeaderFields )( 
            IEmailAction * This,
            /* [retval][out] */ ITaskNamedValueCollection **ppHeaderFields);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_HeaderFields)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_HeaderFields )( 
            IEmailAction * This,
            /* [in] */ ITaskNamedValueCollection *pHeaderFields);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Body)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            IEmailAction * This,
            /* [retval][out] */ BSTR *pBody);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Body)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            IEmailAction * This,
            /* [in] */ BSTR body);
        
        DECLSPEC_XFGVIRT(IEmailAction, get_Attachments)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attachments )( 
            IEmailAction * This,
            /* [retval][out] */ SAFEARRAY * *pAttachements);
        
        DECLSPEC_XFGVIRT(IEmailAction, put_Attachments)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Attachments )( 
            IEmailAction * This,
            /* [max_is][in] */ SAFEARRAY * pAttachements);
        
        END_INTERFACE
    } IEmailActionVtbl;

    interface IEmailAction
    {
        CONST_VTBL struct IEmailActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEmailAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEmailAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEmailAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEmailAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEmailAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEmailAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEmailAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEmailAction_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IEmailAction_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IEmailAction_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 


#define IEmailAction_get_Server(This,pServer)	\
    ( (This)->lpVtbl -> get_Server(This,pServer) ) 

#define IEmailAction_put_Server(This,server)	\
    ( (This)->lpVtbl -> put_Server(This,server) ) 

#define IEmailAction_get_Subject(This,pSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pSubject) ) 

#define IEmailAction_put_Subject(This,subject)	\
    ( (This)->lpVtbl -> put_Subject(This,subject) ) 

#define IEmailAction_get_To(This,pTo)	\
    ( (This)->lpVtbl -> get_To(This,pTo) ) 

#define IEmailAction_put_To(This,to)	\
    ( (This)->lpVtbl -> put_To(This,to) ) 

#define IEmailAction_get_Cc(This,pCc)	\
    ( (This)->lpVtbl -> get_Cc(This,pCc) ) 

#define IEmailAction_put_Cc(This,cc)	\
    ( (This)->lpVtbl -> put_Cc(This,cc) ) 

#define IEmailAction_get_Bcc(This,pBcc)	\
    ( (This)->lpVtbl -> get_Bcc(This,pBcc) ) 

#define IEmailAction_put_Bcc(This,bcc)	\
    ( (This)->lpVtbl -> put_Bcc(This,bcc) ) 

#define IEmailAction_get_ReplyTo(This,pReplyTo)	\
    ( (This)->lpVtbl -> get_ReplyTo(This,pReplyTo) ) 

#define IEmailAction_put_ReplyTo(This,replyTo)	\
    ( (This)->lpVtbl -> put_ReplyTo(This,replyTo) ) 

#define IEmailAction_get_From(This,pFrom)	\
    ( (This)->lpVtbl -> get_From(This,pFrom) ) 

#define IEmailAction_put_From(This,from)	\
    ( (This)->lpVtbl -> put_From(This,from) ) 

#define IEmailAction_get_HeaderFields(This,ppHeaderFields)	\
    ( (This)->lpVtbl -> get_HeaderFields(This,ppHeaderFields) ) 

#define IEmailAction_put_HeaderFields(This,pHeaderFields)	\
    ( (This)->lpVtbl -> put_HeaderFields(This,pHeaderFields) ) 

#define IEmailAction_get_Body(This,pBody)	\
    ( (This)->lpVtbl -> get_Body(This,pBody) ) 

#define IEmailAction_put_Body(This,body)	\
    ( (This)->lpVtbl -> put_Body(This,body) ) 

#define IEmailAction_get_Attachments(This,pAttachements)	\
    ( (This)->lpVtbl -> get_Attachments(This,pAttachements) ) 

#define IEmailAction_put_Attachments(This,pAttachements)	\
    ( (This)->lpVtbl -> put_Attachments(This,pAttachements) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEmailAction_INTERFACE_DEFINED__ */


#ifndef __ITriggerCollection_INTERFACE_DEFINED__
#define __ITriggerCollection_INTERFACE_DEFINED__

/* interface ITriggerCollection */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITriggerCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85df5081-1b24-4f32-878a-d9d14df4cb77")
    ITriggerCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long index,
            /* [retval][out] */ ITrigger **ppTrigger) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **ppEnum) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ TASK_TRIGGER_TYPE2 type,
            /* [retval][out] */ ITrigger **ppTrigger) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT index) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITriggerCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITriggerCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITriggerCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITriggerCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITriggerCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITriggerCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITriggerCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITriggerCollection * This,
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
        
        DECLSPEC_XFGVIRT(ITriggerCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ITriggerCollection * This,
            /* [retval][out] */ long *pCount);
        
        DECLSPEC_XFGVIRT(ITriggerCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            ITriggerCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ ITrigger **ppTrigger);
        
        DECLSPEC_XFGVIRT(ITriggerCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ITriggerCollection * This,
            /* [retval][out] */ IUnknown **ppEnum);
        
        DECLSPEC_XFGVIRT(ITriggerCollection, Create)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            ITriggerCollection * This,
            /* [in] */ TASK_TRIGGER_TYPE2 type,
            /* [retval][out] */ ITrigger **ppTrigger);
        
        DECLSPEC_XFGVIRT(ITriggerCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ITriggerCollection * This,
            /* [in] */ VARIANT index);
        
        DECLSPEC_XFGVIRT(ITriggerCollection, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            ITriggerCollection * This);
        
        END_INTERFACE
    } ITriggerCollectionVtbl;

    interface ITriggerCollection
    {
        CONST_VTBL struct ITriggerCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITriggerCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITriggerCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITriggerCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITriggerCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITriggerCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITriggerCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITriggerCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITriggerCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define ITriggerCollection_get_Item(This,index,ppTrigger)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppTrigger) ) 

#define ITriggerCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define ITriggerCollection_Create(This,type,ppTrigger)	\
    ( (This)->lpVtbl -> Create(This,type,ppTrigger) ) 

#define ITriggerCollection_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#define ITriggerCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITriggerCollection_INTERFACE_DEFINED__ */


#ifndef __IActionCollection_INTERFACE_DEFINED__
#define __IActionCollection_INTERFACE_DEFINED__

/* interface IActionCollection */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IActionCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02820E19-7B98-4ed2-B2E8-FDCCCEFF619B")
    IActionCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long index,
            /* [retval][out] */ IAction **ppAction) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **ppEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_XmlText( 
            /* [retval][out] */ BSTR *pText) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_XmlText( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ TASK_ACTION_TYPE type,
            /* [retval][out] */ IAction **ppAction) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT index) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Context( 
            /* [retval][out] */ BSTR *pContext) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Context( 
            /* [in] */ BSTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActionCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActionCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActionCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActionCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IActionCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IActionCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IActionCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IActionCollection * This,
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
        
        DECLSPEC_XFGVIRT(IActionCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            IActionCollection * This,
            /* [retval][out] */ long *pCount);
        
        DECLSPEC_XFGVIRT(IActionCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            IActionCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ IAction **ppAction);
        
        DECLSPEC_XFGVIRT(IActionCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            IActionCollection * This,
            /* [retval][out] */ IUnknown **ppEnum);
        
        DECLSPEC_XFGVIRT(IActionCollection, get_XmlText)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XmlText )( 
            IActionCollection * This,
            /* [retval][out] */ BSTR *pText);
        
        DECLSPEC_XFGVIRT(IActionCollection, put_XmlText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_XmlText )( 
            IActionCollection * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IActionCollection, Create)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            IActionCollection * This,
            /* [in] */ TASK_ACTION_TYPE type,
            /* [retval][out] */ IAction **ppAction);
        
        DECLSPEC_XFGVIRT(IActionCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IActionCollection * This,
            /* [in] */ VARIANT index);
        
        DECLSPEC_XFGVIRT(IActionCollection, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IActionCollection * This);
        
        DECLSPEC_XFGVIRT(IActionCollection, get_Context)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Context )( 
            IActionCollection * This,
            /* [retval][out] */ BSTR *pContext);
        
        DECLSPEC_XFGVIRT(IActionCollection, put_Context)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Context )( 
            IActionCollection * This,
            /* [in] */ BSTR context);
        
        END_INTERFACE
    } IActionCollectionVtbl;

    interface IActionCollection
    {
        CONST_VTBL struct IActionCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActionCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActionCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActionCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActionCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IActionCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IActionCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IActionCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IActionCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IActionCollection_get_Item(This,index,ppAction)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppAction) ) 

#define IActionCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define IActionCollection_get_XmlText(This,pText)	\
    ( (This)->lpVtbl -> get_XmlText(This,pText) ) 

#define IActionCollection_put_XmlText(This,text)	\
    ( (This)->lpVtbl -> put_XmlText(This,text) ) 

#define IActionCollection_Create(This,type,ppAction)	\
    ( (This)->lpVtbl -> Create(This,type,ppAction) ) 

#define IActionCollection_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#define IActionCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IActionCollection_get_Context(This,pContext)	\
    ( (This)->lpVtbl -> get_Context(This,pContext) ) 

#define IActionCollection_put_Context(This,context)	\
    ( (This)->lpVtbl -> put_Context(This,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActionCollection_INTERFACE_DEFINED__ */


#ifndef __IPrincipal_INTERFACE_DEFINED__
#define __IPrincipal_INTERFACE_DEFINED__

/* interface IPrincipal */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IPrincipal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D98D51E5-C9B4-496a-A9C1-18980261CF0F")
    IPrincipal : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ BSTR *pId) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Id( 
            /* [in] */ BSTR Id) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ BSTR *pName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ BSTR name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UserId( 
            /* [retval][out] */ BSTR *pUser) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UserId( 
            /* [in] */ BSTR user) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LogonType( 
            /* [retval][out] */ TASK_LOGON_TYPE *pLogon) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogonType( 
            /* [in] */ TASK_LOGON_TYPE logon) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_GroupId( 
            /* [retval][out] */ BSTR *pGroup) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_GroupId( 
            /* [in] */ BSTR group) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RunLevel( 
            /* [retval][out] */ TASK_RUNLEVEL_TYPE *pRunLevel) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RunLevel( 
            /* [in] */ TASK_RUNLEVEL_TYPE runLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrincipalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrincipal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrincipal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrincipal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrincipal * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrincipal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrincipal * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrincipal * This,
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
        
        DECLSPEC_XFGVIRT(IPrincipal, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IPrincipal * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            IPrincipal * This,
            /* [in] */ BSTR Id);
        
        DECLSPEC_XFGVIRT(IPrincipal, get_DisplayName)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            IPrincipal * This,
            /* [retval][out] */ BSTR *pName);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_DisplayName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            IPrincipal * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(IPrincipal, get_UserId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserId )( 
            IPrincipal * This,
            /* [retval][out] */ BSTR *pUser);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_UserId)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserId )( 
            IPrincipal * This,
            /* [in] */ BSTR user);
        
        DECLSPEC_XFGVIRT(IPrincipal, get_LogonType)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogonType )( 
            IPrincipal * This,
            /* [retval][out] */ TASK_LOGON_TYPE *pLogon);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_LogonType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogonType )( 
            IPrincipal * This,
            /* [in] */ TASK_LOGON_TYPE logon);
        
        DECLSPEC_XFGVIRT(IPrincipal, get_GroupId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GroupId )( 
            IPrincipal * This,
            /* [retval][out] */ BSTR *pGroup);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_GroupId)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GroupId )( 
            IPrincipal * This,
            /* [in] */ BSTR group);
        
        DECLSPEC_XFGVIRT(IPrincipal, get_RunLevel)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunLevel )( 
            IPrincipal * This,
            /* [retval][out] */ TASK_RUNLEVEL_TYPE *pRunLevel);
        
        DECLSPEC_XFGVIRT(IPrincipal, put_RunLevel)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunLevel )( 
            IPrincipal * This,
            /* [in] */ TASK_RUNLEVEL_TYPE runLevel);
        
        END_INTERFACE
    } IPrincipalVtbl;

    interface IPrincipal
    {
        CONST_VTBL struct IPrincipalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrincipal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrincipal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrincipal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrincipal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrincipal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrincipal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrincipal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrincipal_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define IPrincipal_put_Id(This,Id)	\
    ( (This)->lpVtbl -> put_Id(This,Id) ) 

#define IPrincipal_get_DisplayName(This,pName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pName) ) 

#define IPrincipal_put_DisplayName(This,name)	\
    ( (This)->lpVtbl -> put_DisplayName(This,name) ) 

#define IPrincipal_get_UserId(This,pUser)	\
    ( (This)->lpVtbl -> get_UserId(This,pUser) ) 

#define IPrincipal_put_UserId(This,user)	\
    ( (This)->lpVtbl -> put_UserId(This,user) ) 

#define IPrincipal_get_LogonType(This,pLogon)	\
    ( (This)->lpVtbl -> get_LogonType(This,pLogon) ) 

#define IPrincipal_put_LogonType(This,logon)	\
    ( (This)->lpVtbl -> put_LogonType(This,logon) ) 

#define IPrincipal_get_GroupId(This,pGroup)	\
    ( (This)->lpVtbl -> get_GroupId(This,pGroup) ) 

#define IPrincipal_put_GroupId(This,group)	\
    ( (This)->lpVtbl -> put_GroupId(This,group) ) 

#define IPrincipal_get_RunLevel(This,pRunLevel)	\
    ( (This)->lpVtbl -> get_RunLevel(This,pRunLevel) ) 

#define IPrincipal_put_RunLevel(This,runLevel)	\
    ( (This)->lpVtbl -> put_RunLevel(This,runLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrincipal_INTERFACE_DEFINED__ */


#ifndef __IPrincipal2_INTERFACE_DEFINED__
#define __IPrincipal2_INTERFACE_DEFINED__

/* interface IPrincipal2 */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IPrincipal2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("248919AE-E345-4A6D-8AEB-E0D3165C904E")
    IPrincipal2 : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ProcessTokenSidType( 
            /* [retval][out] */ TASK_PROCESSTOKENSID_TYPE *pProcessTokenSidType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ProcessTokenSidType( 
            /* [in] */ TASK_PROCESSTOKENSID_TYPE processTokenSidType) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RequiredPrivilegeCount( 
            /* [retval][out] */ long *pCount) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RequiredPrivilege( 
            /* [in] */ long index,
            /* [retval][out] */ BSTR *pPrivilege) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddRequiredPrivilege( 
            /* [in] */ BSTR privilege) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrincipal2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrincipal2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrincipal2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrincipal2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrincipal2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrincipal2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrincipal2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrincipal2 * This,
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
        
        DECLSPEC_XFGVIRT(IPrincipal2, get_ProcessTokenSidType)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessTokenSidType )( 
            IPrincipal2 * This,
            /* [retval][out] */ TASK_PROCESSTOKENSID_TYPE *pProcessTokenSidType);
        
        DECLSPEC_XFGVIRT(IPrincipal2, put_ProcessTokenSidType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ProcessTokenSidType )( 
            IPrincipal2 * This,
            /* [in] */ TASK_PROCESSTOKENSID_TYPE processTokenSidType);
        
        DECLSPEC_XFGVIRT(IPrincipal2, get_RequiredPrivilegeCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RequiredPrivilegeCount )( 
            IPrincipal2 * This,
            /* [retval][out] */ long *pCount);
        
        DECLSPEC_XFGVIRT(IPrincipal2, get_RequiredPrivilege)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RequiredPrivilege )( 
            IPrincipal2 * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *pPrivilege);
        
        DECLSPEC_XFGVIRT(IPrincipal2, AddRequiredPrivilege)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddRequiredPrivilege )( 
            IPrincipal2 * This,
            /* [in] */ BSTR privilege);
        
        END_INTERFACE
    } IPrincipal2Vtbl;

    interface IPrincipal2
    {
        CONST_VTBL struct IPrincipal2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrincipal2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrincipal2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrincipal2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrincipal2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrincipal2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrincipal2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrincipal2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrincipal2_get_ProcessTokenSidType(This,pProcessTokenSidType)	\
    ( (This)->lpVtbl -> get_ProcessTokenSidType(This,pProcessTokenSidType) ) 

#define IPrincipal2_put_ProcessTokenSidType(This,processTokenSidType)	\
    ( (This)->lpVtbl -> put_ProcessTokenSidType(This,processTokenSidType) ) 

#define IPrincipal2_get_RequiredPrivilegeCount(This,pCount)	\
    ( (This)->lpVtbl -> get_RequiredPrivilegeCount(This,pCount) ) 

#define IPrincipal2_get_RequiredPrivilege(This,index,pPrivilege)	\
    ( (This)->lpVtbl -> get_RequiredPrivilege(This,index,pPrivilege) ) 

#define IPrincipal2_AddRequiredPrivilege(This,privilege)	\
    ( (This)->lpVtbl -> AddRequiredPrivilege(This,privilege) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrincipal2_INTERFACE_DEFINED__ */


#ifndef __IRegistrationInfo_INTERFACE_DEFINED__
#define __IRegistrationInfo_INTERFACE_DEFINED__

/* interface IRegistrationInfo */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IRegistrationInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("416D8B73-CB41-4ea1-805C-9BE9A5AC4A74")
    IRegistrationInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ BSTR *pDescription) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ BSTR description) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Author( 
            /* [retval][out] */ BSTR *pAuthor) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Author( 
            /* [in] */ BSTR author) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ BSTR *pVersion) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Version( 
            /* [in] */ BSTR version) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Date( 
            /* [retval][out] */ BSTR *pDate) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Date( 
            /* [in] */ BSTR date) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Documentation( 
            /* [retval][out] */ BSTR *pDocumentation) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Documentation( 
            /* [in] */ BSTR documentation) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_XmlText( 
            /* [retval][out] */ BSTR *pText) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_XmlText( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_URI( 
            /* [retval][out] */ BSTR *pUri) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_URI( 
            /* [in] */ BSTR uri) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SecurityDescriptor( 
            /* [retval][out] */ VARIANT *pSddl) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SecurityDescriptor( 
            /* [in] */ VARIANT sddl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ BSTR *pSource) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Source( 
            /* [in] */ BSTR source) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegistrationInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRegistrationInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRegistrationInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRegistrationInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IRegistrationInfo * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IRegistrationInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IRegistrationInfo * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRegistrationInfo * This,
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
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Description)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pDescription);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Description)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR description);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Author)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pAuthor);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Author)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Author )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR author);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Version)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pVersion);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Version)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR version);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Date)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Date )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pDate);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Date)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Date )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR date);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Documentation)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Documentation )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pDocumentation);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Documentation)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Documentation )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR documentation);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_XmlText)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XmlText )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pText);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_XmlText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_XmlText )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_URI)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_URI )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pUri);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_URI)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_URI )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR uri);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_SecurityDescriptor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SecurityDescriptor )( 
            IRegistrationInfo * This,
            /* [retval][out] */ VARIANT *pSddl);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_SecurityDescriptor)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SecurityDescriptor )( 
            IRegistrationInfo * This,
            /* [in] */ VARIANT sddl);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, get_Source)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            IRegistrationInfo * This,
            /* [retval][out] */ BSTR *pSource);
        
        DECLSPEC_XFGVIRT(IRegistrationInfo, put_Source)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            IRegistrationInfo * This,
            /* [in] */ BSTR source);
        
        END_INTERFACE
    } IRegistrationInfoVtbl;

    interface IRegistrationInfo
    {
        CONST_VTBL struct IRegistrationInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegistrationInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegistrationInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegistrationInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegistrationInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRegistrationInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRegistrationInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRegistrationInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRegistrationInfo_get_Description(This,pDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pDescription) ) 

#define IRegistrationInfo_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IRegistrationInfo_get_Author(This,pAuthor)	\
    ( (This)->lpVtbl -> get_Author(This,pAuthor) ) 

#define IRegistrationInfo_put_Author(This,author)	\
    ( (This)->lpVtbl -> put_Author(This,author) ) 

#define IRegistrationInfo_get_Version(This,pVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pVersion) ) 

#define IRegistrationInfo_put_Version(This,version)	\
    ( (This)->lpVtbl -> put_Version(This,version) ) 

#define IRegistrationInfo_get_Date(This,pDate)	\
    ( (This)->lpVtbl -> get_Date(This,pDate) ) 

#define IRegistrationInfo_put_Date(This,date)	\
    ( (This)->lpVtbl -> put_Date(This,date) ) 

#define IRegistrationInfo_get_Documentation(This,pDocumentation)	\
    ( (This)->lpVtbl -> get_Documentation(This,pDocumentation) ) 

#define IRegistrationInfo_put_Documentation(This,documentation)	\
    ( (This)->lpVtbl -> put_Documentation(This,documentation) ) 

#define IRegistrationInfo_get_XmlText(This,pText)	\
    ( (This)->lpVtbl -> get_XmlText(This,pText) ) 

#define IRegistrationInfo_put_XmlText(This,text)	\
    ( (This)->lpVtbl -> put_XmlText(This,text) ) 

#define IRegistrationInfo_get_URI(This,pUri)	\
    ( (This)->lpVtbl -> get_URI(This,pUri) ) 

#define IRegistrationInfo_put_URI(This,uri)	\
    ( (This)->lpVtbl -> put_URI(This,uri) ) 

#define IRegistrationInfo_get_SecurityDescriptor(This,pSddl)	\
    ( (This)->lpVtbl -> get_SecurityDescriptor(This,pSddl) ) 

#define IRegistrationInfo_put_SecurityDescriptor(This,sddl)	\
    ( (This)->lpVtbl -> put_SecurityDescriptor(This,sddl) ) 

#define IRegistrationInfo_get_Source(This,pSource)	\
    ( (This)->lpVtbl -> get_Source(This,pSource) ) 

#define IRegistrationInfo_put_Source(This,source)	\
    ( (This)->lpVtbl -> put_Source(This,source) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegistrationInfo_INTERFACE_DEFINED__ */


#ifndef __ITaskDefinition_INTERFACE_DEFINED__
#define __ITaskDefinition_INTERFACE_DEFINED__

/* interface ITaskDefinition */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskDefinition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f5bc8fc5-536d-4f77-b852-fbc1356fdeb6")
    ITaskDefinition : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RegistrationInfo( 
            /* [retval][out] */ IRegistrationInfo **ppRegistrationInfo) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RegistrationInfo( 
            /* [in] */ IRegistrationInfo *pRegistrationInfo) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Triggers( 
            /* [retval][out] */ ITriggerCollection **ppTriggers) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Triggers( 
            /* [in] */ ITriggerCollection *pTriggers) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Settings( 
            /* [retval][out] */ ITaskSettings **ppSettings) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Settings( 
            /* [in] */ ITaskSettings *pSettings) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ BSTR *pData) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ BSTR data) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Principal( 
            /* [retval][out] */ IPrincipal **ppPrincipal) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Principal( 
            /* [in] */ IPrincipal *pPrincipal) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Actions( 
            /* [retval][out] */ IActionCollection **ppActions) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Actions( 
            /* [in] */ IActionCollection *pActions) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_XmlText( 
            /* [retval][out] */ BSTR *pXml) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_XmlText( 
            /* [in] */ BSTR xml) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskDefinitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskDefinition * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskDefinition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskDefinition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskDefinition * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskDefinition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskDefinition * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskDefinition * This,
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
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_RegistrationInfo)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RegistrationInfo )( 
            ITaskDefinition * This,
            /* [retval][out] */ IRegistrationInfo **ppRegistrationInfo);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_RegistrationInfo)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RegistrationInfo )( 
            ITaskDefinition * This,
            /* [in] */ IRegistrationInfo *pRegistrationInfo);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_Triggers)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Triggers )( 
            ITaskDefinition * This,
            /* [retval][out] */ ITriggerCollection **ppTriggers);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_Triggers)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Triggers )( 
            ITaskDefinition * This,
            /* [in] */ ITriggerCollection *pTriggers);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_Settings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Settings )( 
            ITaskDefinition * This,
            /* [retval][out] */ ITaskSettings **ppSettings);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_Settings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Settings )( 
            ITaskDefinition * This,
            /* [in] */ ITaskSettings *pSettings);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_Data)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            ITaskDefinition * This,
            /* [retval][out] */ BSTR *pData);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_Data)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            ITaskDefinition * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_Principal)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Principal )( 
            ITaskDefinition * This,
            /* [retval][out] */ IPrincipal **ppPrincipal);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_Principal)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Principal )( 
            ITaskDefinition * This,
            /* [in] */ IPrincipal *pPrincipal);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_Actions)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Actions )( 
            ITaskDefinition * This,
            /* [retval][out] */ IActionCollection **ppActions);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_Actions)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Actions )( 
            ITaskDefinition * This,
            /* [in] */ IActionCollection *pActions);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, get_XmlText)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XmlText )( 
            ITaskDefinition * This,
            /* [retval][out] */ BSTR *pXml);
        
        DECLSPEC_XFGVIRT(ITaskDefinition, put_XmlText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_XmlText )( 
            ITaskDefinition * This,
            /* [in] */ BSTR xml);
        
        END_INTERFACE
    } ITaskDefinitionVtbl;

    interface ITaskDefinition
    {
        CONST_VTBL struct ITaskDefinitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskDefinition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskDefinition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskDefinition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskDefinition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskDefinition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskDefinition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskDefinition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskDefinition_get_RegistrationInfo(This,ppRegistrationInfo)	\
    ( (This)->lpVtbl -> get_RegistrationInfo(This,ppRegistrationInfo) ) 

#define ITaskDefinition_put_RegistrationInfo(This,pRegistrationInfo)	\
    ( (This)->lpVtbl -> put_RegistrationInfo(This,pRegistrationInfo) ) 

#define ITaskDefinition_get_Triggers(This,ppTriggers)	\
    ( (This)->lpVtbl -> get_Triggers(This,ppTriggers) ) 

#define ITaskDefinition_put_Triggers(This,pTriggers)	\
    ( (This)->lpVtbl -> put_Triggers(This,pTriggers) ) 

#define ITaskDefinition_get_Settings(This,ppSettings)	\
    ( (This)->lpVtbl -> get_Settings(This,ppSettings) ) 

#define ITaskDefinition_put_Settings(This,pSettings)	\
    ( (This)->lpVtbl -> put_Settings(This,pSettings) ) 

#define ITaskDefinition_get_Data(This,pData)	\
    ( (This)->lpVtbl -> get_Data(This,pData) ) 

#define ITaskDefinition_put_Data(This,data)	\
    ( (This)->lpVtbl -> put_Data(This,data) ) 

#define ITaskDefinition_get_Principal(This,ppPrincipal)	\
    ( (This)->lpVtbl -> get_Principal(This,ppPrincipal) ) 

#define ITaskDefinition_put_Principal(This,pPrincipal)	\
    ( (This)->lpVtbl -> put_Principal(This,pPrincipal) ) 

#define ITaskDefinition_get_Actions(This,ppActions)	\
    ( (This)->lpVtbl -> get_Actions(This,ppActions) ) 

#define ITaskDefinition_put_Actions(This,pActions)	\
    ( (This)->lpVtbl -> put_Actions(This,pActions) ) 

#define ITaskDefinition_get_XmlText(This,pXml)	\
    ( (This)->lpVtbl -> get_XmlText(This,pXml) ) 

#define ITaskDefinition_put_XmlText(This,xml)	\
    ( (This)->lpVtbl -> put_XmlText(This,xml) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskDefinition_INTERFACE_DEFINED__ */


#ifndef __ITaskSettings_INTERFACE_DEFINED__
#define __ITaskSettings_INTERFACE_DEFINED__

/* interface ITaskSettings */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8FD4711D-2D02-4c8c-87E3-EFF699DE127E")
    ITaskSettings : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AllowDemandStart( 
            /* [retval][out] */ VARIANT_BOOL *pAllowDemandStart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AllowDemandStart( 
            /* [in] */ VARIANT_BOOL allowDemandStart) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RestartInterval( 
            /* [retval][out] */ BSTR *pRestartInterval) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RestartInterval( 
            /* [in] */ BSTR restartInterval) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RestartCount( 
            /* [retval][out] */ int *pRestartCount) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RestartCount( 
            /* [in] */ int restartCount) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MultipleInstances( 
            /* [retval][out] */ TASK_INSTANCES_POLICY *pPolicy) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MultipleInstances( 
            /* [in] */ TASK_INSTANCES_POLICY policy) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StopIfGoingOnBatteries( 
            /* [retval][out] */ VARIANT_BOOL *pStopIfOnBatteries) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StopIfGoingOnBatteries( 
            /* [in] */ VARIANT_BOOL stopIfOnBatteries) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisallowStartIfOnBatteries( 
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisallowStartIfOnBatteries( 
            /* [in] */ VARIANT_BOOL disallowStart) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AllowHardTerminate( 
            /* [retval][out] */ VARIANT_BOOL *pAllowHardTerminate) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AllowHardTerminate( 
            /* [in] */ VARIANT_BOOL allowHardTerminate) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StartWhenAvailable( 
            /* [retval][out] */ VARIANT_BOOL *pStartWhenAvailable) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StartWhenAvailable( 
            /* [in] */ VARIANT_BOOL startWhenAvailable) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_XmlText( 
            /* [retval][out] */ BSTR *pText) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_XmlText( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RunOnlyIfNetworkAvailable( 
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfNetworkAvailable) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RunOnlyIfNetworkAvailable( 
            /* [in] */ VARIANT_BOOL runOnlyIfNetworkAvailable) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ExecutionTimeLimit( 
            /* [retval][out] */ BSTR *pExecutionTimeLimit) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ExecutionTimeLimit( 
            /* [in] */ BSTR executionTimeLimit) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ VARIANT_BOOL *pEnabled) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DeleteExpiredTaskAfter( 
            /* [retval][out] */ BSTR *pExpirationDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DeleteExpiredTaskAfter( 
            /* [in] */ BSTR expirationDelay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ int *pPriority) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ int priority) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Compatibility( 
            /* [retval][out] */ TASK_COMPATIBILITY *pCompatLevel) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Compatibility( 
            /* [in] */ TASK_COMPATIBILITY compatLevel) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Hidden( 
            /* [retval][out] */ VARIANT_BOOL *pHidden) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Hidden( 
            /* [in] */ VARIANT_BOOL hidden) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IdleSettings( 
            /* [retval][out] */ IIdleSettings **ppIdleSettings) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_IdleSettings( 
            /* [in] */ IIdleSettings *pIdleSettings) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RunOnlyIfIdle( 
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfIdle) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RunOnlyIfIdle( 
            /* [in] */ VARIANT_BOOL runOnlyIfIdle) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WakeToRun( 
            /* [retval][out] */ VARIANT_BOOL *pWake) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_WakeToRun( 
            /* [in] */ VARIANT_BOOL wake) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_NetworkSettings( 
            /* [retval][out] */ INetworkSettings **ppNetworkSettings) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_NetworkSettings( 
            /* [in] */ INetworkSettings *pNetworkSettings) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskSettings * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskSettings * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskSettings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskSettings * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskSettings * This,
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
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_AllowDemandStart)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllowDemandStart )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pAllowDemandStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_AllowDemandStart)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AllowDemandStart )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL allowDemandStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RestartInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RestartInterval )( 
            ITaskSettings * This,
            /* [retval][out] */ BSTR *pRestartInterval);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RestartInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RestartInterval )( 
            ITaskSettings * This,
            /* [in] */ BSTR restartInterval);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RestartCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RestartCount )( 
            ITaskSettings * This,
            /* [retval][out] */ int *pRestartCount);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RestartCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RestartCount )( 
            ITaskSettings * This,
            /* [in] */ int restartCount);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_MultipleInstances)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MultipleInstances )( 
            ITaskSettings * This,
            /* [retval][out] */ TASK_INSTANCES_POLICY *pPolicy);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_MultipleInstances)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MultipleInstances )( 
            ITaskSettings * This,
            /* [in] */ TASK_INSTANCES_POLICY policy);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_StopIfGoingOnBatteries)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StopIfGoingOnBatteries )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pStopIfOnBatteries);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_StopIfGoingOnBatteries)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StopIfGoingOnBatteries )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL stopIfOnBatteries);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_DisallowStartIfOnBatteries)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisallowStartIfOnBatteries )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_DisallowStartIfOnBatteries)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisallowStartIfOnBatteries )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL disallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_AllowHardTerminate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllowHardTerminate )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pAllowHardTerminate);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_AllowHardTerminate)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AllowHardTerminate )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL allowHardTerminate);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_StartWhenAvailable)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartWhenAvailable )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pStartWhenAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_StartWhenAvailable)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartWhenAvailable )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL startWhenAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_XmlText)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XmlText )( 
            ITaskSettings * This,
            /* [retval][out] */ BSTR *pText);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_XmlText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_XmlText )( 
            ITaskSettings * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RunOnlyIfNetworkAvailable)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnlyIfNetworkAvailable )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfNetworkAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RunOnlyIfNetworkAvailable)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnlyIfNetworkAvailable )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL runOnlyIfNetworkAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ITaskSettings * This,
            /* [retval][out] */ BSTR *pExecutionTimeLimit);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ITaskSettings * This,
            /* [in] */ BSTR executionTimeLimit);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_DeleteExpiredTaskAfter)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteExpiredTaskAfter )( 
            ITaskSettings * This,
            /* [retval][out] */ BSTR *pExpirationDelay);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_DeleteExpiredTaskAfter)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteExpiredTaskAfter )( 
            ITaskSettings * This,
            /* [in] */ BSTR expirationDelay);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Priority)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            ITaskSettings * This,
            /* [retval][out] */ int *pPriority);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Priority)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            ITaskSettings * This,
            /* [in] */ int priority);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Compatibility)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Compatibility )( 
            ITaskSettings * This,
            /* [retval][out] */ TASK_COMPATIBILITY *pCompatLevel);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Compatibility)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Compatibility )( 
            ITaskSettings * This,
            /* [in] */ TASK_COMPATIBILITY compatLevel);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Hidden)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Hidden )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pHidden);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Hidden)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Hidden )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL hidden);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_IdleSettings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IdleSettings )( 
            ITaskSettings * This,
            /* [retval][out] */ IIdleSettings **ppIdleSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_IdleSettings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IdleSettings )( 
            ITaskSettings * This,
            /* [in] */ IIdleSettings *pIdleSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RunOnlyIfIdle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnlyIfIdle )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfIdle);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RunOnlyIfIdle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnlyIfIdle )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL runOnlyIfIdle);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_WakeToRun)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WakeToRun )( 
            ITaskSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pWake);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_WakeToRun)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WakeToRun )( 
            ITaskSettings * This,
            /* [in] */ VARIANT_BOOL wake);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_NetworkSettings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkSettings )( 
            ITaskSettings * This,
            /* [retval][out] */ INetworkSettings **ppNetworkSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_NetworkSettings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkSettings )( 
            ITaskSettings * This,
            /* [in] */ INetworkSettings *pNetworkSettings);
        
        END_INTERFACE
    } ITaskSettingsVtbl;

    interface ITaskSettings
    {
        CONST_VTBL struct ITaskSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskSettings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskSettings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskSettings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskSettings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskSettings_get_AllowDemandStart(This,pAllowDemandStart)	\
    ( (This)->lpVtbl -> get_AllowDemandStart(This,pAllowDemandStart) ) 

#define ITaskSettings_put_AllowDemandStart(This,allowDemandStart)	\
    ( (This)->lpVtbl -> put_AllowDemandStart(This,allowDemandStart) ) 

#define ITaskSettings_get_RestartInterval(This,pRestartInterval)	\
    ( (This)->lpVtbl -> get_RestartInterval(This,pRestartInterval) ) 

#define ITaskSettings_put_RestartInterval(This,restartInterval)	\
    ( (This)->lpVtbl -> put_RestartInterval(This,restartInterval) ) 

#define ITaskSettings_get_RestartCount(This,pRestartCount)	\
    ( (This)->lpVtbl -> get_RestartCount(This,pRestartCount) ) 

#define ITaskSettings_put_RestartCount(This,restartCount)	\
    ( (This)->lpVtbl -> put_RestartCount(This,restartCount) ) 

#define ITaskSettings_get_MultipleInstances(This,pPolicy)	\
    ( (This)->lpVtbl -> get_MultipleInstances(This,pPolicy) ) 

#define ITaskSettings_put_MultipleInstances(This,policy)	\
    ( (This)->lpVtbl -> put_MultipleInstances(This,policy) ) 

#define ITaskSettings_get_StopIfGoingOnBatteries(This,pStopIfOnBatteries)	\
    ( (This)->lpVtbl -> get_StopIfGoingOnBatteries(This,pStopIfOnBatteries) ) 

#define ITaskSettings_put_StopIfGoingOnBatteries(This,stopIfOnBatteries)	\
    ( (This)->lpVtbl -> put_StopIfGoingOnBatteries(This,stopIfOnBatteries) ) 

#define ITaskSettings_get_DisallowStartIfOnBatteries(This,pDisallowStart)	\
    ( (This)->lpVtbl -> get_DisallowStartIfOnBatteries(This,pDisallowStart) ) 

#define ITaskSettings_put_DisallowStartIfOnBatteries(This,disallowStart)	\
    ( (This)->lpVtbl -> put_DisallowStartIfOnBatteries(This,disallowStart) ) 

#define ITaskSettings_get_AllowHardTerminate(This,pAllowHardTerminate)	\
    ( (This)->lpVtbl -> get_AllowHardTerminate(This,pAllowHardTerminate) ) 

#define ITaskSettings_put_AllowHardTerminate(This,allowHardTerminate)	\
    ( (This)->lpVtbl -> put_AllowHardTerminate(This,allowHardTerminate) ) 

#define ITaskSettings_get_StartWhenAvailable(This,pStartWhenAvailable)	\
    ( (This)->lpVtbl -> get_StartWhenAvailable(This,pStartWhenAvailable) ) 

#define ITaskSettings_put_StartWhenAvailable(This,startWhenAvailable)	\
    ( (This)->lpVtbl -> put_StartWhenAvailable(This,startWhenAvailable) ) 

#define ITaskSettings_get_XmlText(This,pText)	\
    ( (This)->lpVtbl -> get_XmlText(This,pText) ) 

#define ITaskSettings_put_XmlText(This,text)	\
    ( (This)->lpVtbl -> put_XmlText(This,text) ) 

#define ITaskSettings_get_RunOnlyIfNetworkAvailable(This,pRunOnlyIfNetworkAvailable)	\
    ( (This)->lpVtbl -> get_RunOnlyIfNetworkAvailable(This,pRunOnlyIfNetworkAvailable) ) 

#define ITaskSettings_put_RunOnlyIfNetworkAvailable(This,runOnlyIfNetworkAvailable)	\
    ( (This)->lpVtbl -> put_RunOnlyIfNetworkAvailable(This,runOnlyIfNetworkAvailable) ) 

#define ITaskSettings_get_ExecutionTimeLimit(This,pExecutionTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pExecutionTimeLimit) ) 

#define ITaskSettings_put_ExecutionTimeLimit(This,executionTimeLimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,executionTimeLimit) ) 

#define ITaskSettings_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ITaskSettings_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define ITaskSettings_get_DeleteExpiredTaskAfter(This,pExpirationDelay)	\
    ( (This)->lpVtbl -> get_DeleteExpiredTaskAfter(This,pExpirationDelay) ) 

#define ITaskSettings_put_DeleteExpiredTaskAfter(This,expirationDelay)	\
    ( (This)->lpVtbl -> put_DeleteExpiredTaskAfter(This,expirationDelay) ) 

#define ITaskSettings_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define ITaskSettings_put_Priority(This,priority)	\
    ( (This)->lpVtbl -> put_Priority(This,priority) ) 

#define ITaskSettings_get_Compatibility(This,pCompatLevel)	\
    ( (This)->lpVtbl -> get_Compatibility(This,pCompatLevel) ) 

#define ITaskSettings_put_Compatibility(This,compatLevel)	\
    ( (This)->lpVtbl -> put_Compatibility(This,compatLevel) ) 

#define ITaskSettings_get_Hidden(This,pHidden)	\
    ( (This)->lpVtbl -> get_Hidden(This,pHidden) ) 

#define ITaskSettings_put_Hidden(This,hidden)	\
    ( (This)->lpVtbl -> put_Hidden(This,hidden) ) 

#define ITaskSettings_get_IdleSettings(This,ppIdleSettings)	\
    ( (This)->lpVtbl -> get_IdleSettings(This,ppIdleSettings) ) 

#define ITaskSettings_put_IdleSettings(This,pIdleSettings)	\
    ( (This)->lpVtbl -> put_IdleSettings(This,pIdleSettings) ) 

#define ITaskSettings_get_RunOnlyIfIdle(This,pRunOnlyIfIdle)	\
    ( (This)->lpVtbl -> get_RunOnlyIfIdle(This,pRunOnlyIfIdle) ) 

#define ITaskSettings_put_RunOnlyIfIdle(This,runOnlyIfIdle)	\
    ( (This)->lpVtbl -> put_RunOnlyIfIdle(This,runOnlyIfIdle) ) 

#define ITaskSettings_get_WakeToRun(This,pWake)	\
    ( (This)->lpVtbl -> get_WakeToRun(This,pWake) ) 

#define ITaskSettings_put_WakeToRun(This,wake)	\
    ( (This)->lpVtbl -> put_WakeToRun(This,wake) ) 

#define ITaskSettings_get_NetworkSettings(This,ppNetworkSettings)	\
    ( (This)->lpVtbl -> get_NetworkSettings(This,ppNetworkSettings) ) 

#define ITaskSettings_put_NetworkSettings(This,pNetworkSettings)	\
    ( (This)->lpVtbl -> put_NetworkSettings(This,pNetworkSettings) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskSettings_INTERFACE_DEFINED__ */


#ifndef __ITaskSettings2_INTERFACE_DEFINED__
#define __ITaskSettings2_INTERFACE_DEFINED__

/* interface ITaskSettings2 */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskSettings2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C05C3F0-6EED-4c05-A15F-ED7D7A98A369")
    ITaskSettings2 : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisallowStartOnRemoteAppSession( 
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisallowStartOnRemoteAppSession( 
            /* [in] */ VARIANT_BOOL disallowStart) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UseUnifiedSchedulingEngine( 
            /* [retval][out] */ VARIANT_BOOL *pUseUnifiedEngine) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UseUnifiedSchedulingEngine( 
            /* [in] */ VARIANT_BOOL useUnifiedEngine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskSettings2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskSettings2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskSettings2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskSettings2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskSettings2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskSettings2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskSettings2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskSettings2 * This,
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
        
        DECLSPEC_XFGVIRT(ITaskSettings2, get_DisallowStartOnRemoteAppSession)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisallowStartOnRemoteAppSession )( 
            ITaskSettings2 * This,
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings2, put_DisallowStartOnRemoteAppSession)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisallowStartOnRemoteAppSession )( 
            ITaskSettings2 * This,
            /* [in] */ VARIANT_BOOL disallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings2, get_UseUnifiedSchedulingEngine)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UseUnifiedSchedulingEngine )( 
            ITaskSettings2 * This,
            /* [retval][out] */ VARIANT_BOOL *pUseUnifiedEngine);
        
        DECLSPEC_XFGVIRT(ITaskSettings2, put_UseUnifiedSchedulingEngine)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UseUnifiedSchedulingEngine )( 
            ITaskSettings2 * This,
            /* [in] */ VARIANT_BOOL useUnifiedEngine);
        
        END_INTERFACE
    } ITaskSettings2Vtbl;

    interface ITaskSettings2
    {
        CONST_VTBL struct ITaskSettings2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskSettings2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskSettings2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskSettings2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskSettings2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskSettings2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskSettings2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskSettings2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskSettings2_get_DisallowStartOnRemoteAppSession(This,pDisallowStart)	\
    ( (This)->lpVtbl -> get_DisallowStartOnRemoteAppSession(This,pDisallowStart) ) 

#define ITaskSettings2_put_DisallowStartOnRemoteAppSession(This,disallowStart)	\
    ( (This)->lpVtbl -> put_DisallowStartOnRemoteAppSession(This,disallowStart) ) 

#define ITaskSettings2_get_UseUnifiedSchedulingEngine(This,pUseUnifiedEngine)	\
    ( (This)->lpVtbl -> get_UseUnifiedSchedulingEngine(This,pUseUnifiedEngine) ) 

#define ITaskSettings2_put_UseUnifiedSchedulingEngine(This,useUnifiedEngine)	\
    ( (This)->lpVtbl -> put_UseUnifiedSchedulingEngine(This,useUnifiedEngine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskSettings2_INTERFACE_DEFINED__ */


#ifndef __ITaskSettings3_INTERFACE_DEFINED__
#define __ITaskSettings3_INTERFACE_DEFINED__

/* interface ITaskSettings3 */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ITaskSettings3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AD9D0D7-0C7F-4EBB-9A5F-D1C648DCA528")
    ITaskSettings3 : public ITaskSettings
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisallowStartOnRemoteAppSession( 
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisallowStartOnRemoteAppSession( 
            /* [in] */ VARIANT_BOOL disallowStart) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UseUnifiedSchedulingEngine( 
            /* [retval][out] */ VARIANT_BOOL *pUseUnifiedEngine) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UseUnifiedSchedulingEngine( 
            /* [in] */ VARIANT_BOOL useUnifiedEngine) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MaintenanceSettings( 
            /* [retval][out] */ IMaintenanceSettings **ppMaintenanceSettings) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaintenanceSettings( 
            /* [in] */ IMaintenanceSettings *pMaintenanceSettings) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateMaintenanceSettings( 
            /* [retval][out] */ IMaintenanceSettings **ppMaintenanceSettings) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Volatile( 
            /* [retval][out] */ VARIANT_BOOL *pVolatile) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Volatile( 
            /* [in] */ VARIANT_BOOL Volatile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskSettings3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITaskSettings3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITaskSettings3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITaskSettings3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITaskSettings3 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ITaskSettings3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITaskSettings3 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskSettings3 * This,
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
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_AllowDemandStart)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllowDemandStart )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pAllowDemandStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_AllowDemandStart)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AllowDemandStart )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL allowDemandStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RestartInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RestartInterval )( 
            ITaskSettings3 * This,
            /* [retval][out] */ BSTR *pRestartInterval);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RestartInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RestartInterval )( 
            ITaskSettings3 * This,
            /* [in] */ BSTR restartInterval);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RestartCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RestartCount )( 
            ITaskSettings3 * This,
            /* [retval][out] */ int *pRestartCount);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RestartCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RestartCount )( 
            ITaskSettings3 * This,
            /* [in] */ int restartCount);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_MultipleInstances)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MultipleInstances )( 
            ITaskSettings3 * This,
            /* [retval][out] */ TASK_INSTANCES_POLICY *pPolicy);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_MultipleInstances)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MultipleInstances )( 
            ITaskSettings3 * This,
            /* [in] */ TASK_INSTANCES_POLICY policy);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_StopIfGoingOnBatteries)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StopIfGoingOnBatteries )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pStopIfOnBatteries);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_StopIfGoingOnBatteries)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StopIfGoingOnBatteries )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL stopIfOnBatteries);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_DisallowStartIfOnBatteries)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisallowStartIfOnBatteries )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_DisallowStartIfOnBatteries)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisallowStartIfOnBatteries )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL disallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_AllowHardTerminate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllowHardTerminate )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pAllowHardTerminate);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_AllowHardTerminate)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AllowHardTerminate )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL allowHardTerminate);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_StartWhenAvailable)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StartWhenAvailable )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pStartWhenAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_StartWhenAvailable)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StartWhenAvailable )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL startWhenAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_XmlText)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XmlText )( 
            ITaskSettings3 * This,
            /* [retval][out] */ BSTR *pText);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_XmlText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_XmlText )( 
            ITaskSettings3 * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RunOnlyIfNetworkAvailable)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnlyIfNetworkAvailable )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfNetworkAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RunOnlyIfNetworkAvailable)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnlyIfNetworkAvailable )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL runOnlyIfNetworkAvailable);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_ExecutionTimeLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutionTimeLimit )( 
            ITaskSettings3 * This,
            /* [retval][out] */ BSTR *pExecutionTimeLimit);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_ExecutionTimeLimit)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutionTimeLimit )( 
            ITaskSettings3 * This,
            /* [in] */ BSTR executionTimeLimit);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Enabled)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_DeleteExpiredTaskAfter)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteExpiredTaskAfter )( 
            ITaskSettings3 * This,
            /* [retval][out] */ BSTR *pExpirationDelay);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_DeleteExpiredTaskAfter)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteExpiredTaskAfter )( 
            ITaskSettings3 * This,
            /* [in] */ BSTR expirationDelay);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Priority)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            ITaskSettings3 * This,
            /* [retval][out] */ int *pPriority);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Priority)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            ITaskSettings3 * This,
            /* [in] */ int priority);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Compatibility)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Compatibility )( 
            ITaskSettings3 * This,
            /* [retval][out] */ TASK_COMPATIBILITY *pCompatLevel);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Compatibility)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Compatibility )( 
            ITaskSettings3 * This,
            /* [in] */ TASK_COMPATIBILITY compatLevel);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_Hidden)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Hidden )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pHidden);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_Hidden)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Hidden )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL hidden);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_IdleSettings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IdleSettings )( 
            ITaskSettings3 * This,
            /* [retval][out] */ IIdleSettings **ppIdleSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_IdleSettings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IdleSettings )( 
            ITaskSettings3 * This,
            /* [in] */ IIdleSettings *pIdleSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_RunOnlyIfIdle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RunOnlyIfIdle )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pRunOnlyIfIdle);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_RunOnlyIfIdle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RunOnlyIfIdle )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL runOnlyIfIdle);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_WakeToRun)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WakeToRun )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pWake);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_WakeToRun)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WakeToRun )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL wake);
        
        DECLSPEC_XFGVIRT(ITaskSettings, get_NetworkSettings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkSettings )( 
            ITaskSettings3 * This,
            /* [retval][out] */ INetworkSettings **ppNetworkSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings, put_NetworkSettings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkSettings )( 
            ITaskSettings3 * This,
            /* [in] */ INetworkSettings *pNetworkSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, get_DisallowStartOnRemoteAppSession)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisallowStartOnRemoteAppSession )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pDisallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, put_DisallowStartOnRemoteAppSession)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisallowStartOnRemoteAppSession )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL disallowStart);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, get_UseUnifiedSchedulingEngine)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UseUnifiedSchedulingEngine )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pUseUnifiedEngine);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, put_UseUnifiedSchedulingEngine)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UseUnifiedSchedulingEngine )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL useUnifiedEngine);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, get_MaintenanceSettings)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaintenanceSettings )( 
            ITaskSettings3 * This,
            /* [retval][out] */ IMaintenanceSettings **ppMaintenanceSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, put_MaintenanceSettings)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaintenanceSettings )( 
            ITaskSettings3 * This,
            /* [in] */ IMaintenanceSettings *pMaintenanceSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, CreateMaintenanceSettings)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateMaintenanceSettings )( 
            ITaskSettings3 * This,
            /* [retval][out] */ IMaintenanceSettings **ppMaintenanceSettings);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, get_Volatile)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Volatile )( 
            ITaskSettings3 * This,
            /* [retval][out] */ VARIANT_BOOL *pVolatile);
        
        DECLSPEC_XFGVIRT(ITaskSettings3, put_Volatile)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Volatile )( 
            ITaskSettings3 * This,
            /* [in] */ VARIANT_BOOL Volatile);
        
        END_INTERFACE
    } ITaskSettings3Vtbl;

    interface ITaskSettings3
    {
        CONST_VTBL struct ITaskSettings3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskSettings3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskSettings3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskSettings3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskSettings3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskSettings3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskSettings3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskSettings3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskSettings3_get_AllowDemandStart(This,pAllowDemandStart)	\
    ( (This)->lpVtbl -> get_AllowDemandStart(This,pAllowDemandStart) ) 

#define ITaskSettings3_put_AllowDemandStart(This,allowDemandStart)	\
    ( (This)->lpVtbl -> put_AllowDemandStart(This,allowDemandStart) ) 

#define ITaskSettings3_get_RestartInterval(This,pRestartInterval)	\
    ( (This)->lpVtbl -> get_RestartInterval(This,pRestartInterval) ) 

#define ITaskSettings3_put_RestartInterval(This,restartInterval)	\
    ( (This)->lpVtbl -> put_RestartInterval(This,restartInterval) ) 

#define ITaskSettings3_get_RestartCount(This,pRestartCount)	\
    ( (This)->lpVtbl -> get_RestartCount(This,pRestartCount) ) 

#define ITaskSettings3_put_RestartCount(This,restartCount)	\
    ( (This)->lpVtbl -> put_RestartCount(This,restartCount) ) 

#define ITaskSettings3_get_MultipleInstances(This,pPolicy)	\
    ( (This)->lpVtbl -> get_MultipleInstances(This,pPolicy) ) 

#define ITaskSettings3_put_MultipleInstances(This,policy)	\
    ( (This)->lpVtbl -> put_MultipleInstances(This,policy) ) 

#define ITaskSettings3_get_StopIfGoingOnBatteries(This,pStopIfOnBatteries)	\
    ( (This)->lpVtbl -> get_StopIfGoingOnBatteries(This,pStopIfOnBatteries) ) 

#define ITaskSettings3_put_StopIfGoingOnBatteries(This,stopIfOnBatteries)	\
    ( (This)->lpVtbl -> put_StopIfGoingOnBatteries(This,stopIfOnBatteries) ) 

#define ITaskSettings3_get_DisallowStartIfOnBatteries(This,pDisallowStart)	\
    ( (This)->lpVtbl -> get_DisallowStartIfOnBatteries(This,pDisallowStart) ) 

#define ITaskSettings3_put_DisallowStartIfOnBatteries(This,disallowStart)	\
    ( (This)->lpVtbl -> put_DisallowStartIfOnBatteries(This,disallowStart) ) 

#define ITaskSettings3_get_AllowHardTerminate(This,pAllowHardTerminate)	\
    ( (This)->lpVtbl -> get_AllowHardTerminate(This,pAllowHardTerminate) ) 

#define ITaskSettings3_put_AllowHardTerminate(This,allowHardTerminate)	\
    ( (This)->lpVtbl -> put_AllowHardTerminate(This,allowHardTerminate) ) 

#define ITaskSettings3_get_StartWhenAvailable(This,pStartWhenAvailable)	\
    ( (This)->lpVtbl -> get_StartWhenAvailable(This,pStartWhenAvailable) ) 

#define ITaskSettings3_put_StartWhenAvailable(This,startWhenAvailable)	\
    ( (This)->lpVtbl -> put_StartWhenAvailable(This,startWhenAvailable) ) 

#define ITaskSettings3_get_XmlText(This,pText)	\
    ( (This)->lpVtbl -> get_XmlText(This,pText) ) 

#define ITaskSettings3_put_XmlText(This,text)	\
    ( (This)->lpVtbl -> put_XmlText(This,text) ) 

#define ITaskSettings3_get_RunOnlyIfNetworkAvailable(This,pRunOnlyIfNetworkAvailable)	\
    ( (This)->lpVtbl -> get_RunOnlyIfNetworkAvailable(This,pRunOnlyIfNetworkAvailable) ) 

#define ITaskSettings3_put_RunOnlyIfNetworkAvailable(This,runOnlyIfNetworkAvailable)	\
    ( (This)->lpVtbl -> put_RunOnlyIfNetworkAvailable(This,runOnlyIfNetworkAvailable) ) 

#define ITaskSettings3_get_ExecutionTimeLimit(This,pExecutionTimeLimit)	\
    ( (This)->lpVtbl -> get_ExecutionTimeLimit(This,pExecutionTimeLimit) ) 

#define ITaskSettings3_put_ExecutionTimeLimit(This,executionTimeLimit)	\
    ( (This)->lpVtbl -> put_ExecutionTimeLimit(This,executionTimeLimit) ) 

#define ITaskSettings3_get_Enabled(This,pEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pEnabled) ) 

#define ITaskSettings3_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define ITaskSettings3_get_DeleteExpiredTaskAfter(This,pExpirationDelay)	\
    ( (This)->lpVtbl -> get_DeleteExpiredTaskAfter(This,pExpirationDelay) ) 

#define ITaskSettings3_put_DeleteExpiredTaskAfter(This,expirationDelay)	\
    ( (This)->lpVtbl -> put_DeleteExpiredTaskAfter(This,expirationDelay) ) 

#define ITaskSettings3_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define ITaskSettings3_put_Priority(This,priority)	\
    ( (This)->lpVtbl -> put_Priority(This,priority) ) 

#define ITaskSettings3_get_Compatibility(This,pCompatLevel)	\
    ( (This)->lpVtbl -> get_Compatibility(This,pCompatLevel) ) 

#define ITaskSettings3_put_Compatibility(This,compatLevel)	\
    ( (This)->lpVtbl -> put_Compatibility(This,compatLevel) ) 

#define ITaskSettings3_get_Hidden(This,pHidden)	\
    ( (This)->lpVtbl -> get_Hidden(This,pHidden) ) 

#define ITaskSettings3_put_Hidden(This,hidden)	\
    ( (This)->lpVtbl -> put_Hidden(This,hidden) ) 

#define ITaskSettings3_get_IdleSettings(This,ppIdleSettings)	\
    ( (This)->lpVtbl -> get_IdleSettings(This,ppIdleSettings) ) 

#define ITaskSettings3_put_IdleSettings(This,pIdleSettings)	\
    ( (This)->lpVtbl -> put_IdleSettings(This,pIdleSettings) ) 

#define ITaskSettings3_get_RunOnlyIfIdle(This,pRunOnlyIfIdle)	\
    ( (This)->lpVtbl -> get_RunOnlyIfIdle(This,pRunOnlyIfIdle) ) 

#define ITaskSettings3_put_RunOnlyIfIdle(This,runOnlyIfIdle)	\
    ( (This)->lpVtbl -> put_RunOnlyIfIdle(This,runOnlyIfIdle) ) 

#define ITaskSettings3_get_WakeToRun(This,pWake)	\
    ( (This)->lpVtbl -> get_WakeToRun(This,pWake) ) 

#define ITaskSettings3_put_WakeToRun(This,wake)	\
    ( (This)->lpVtbl -> put_WakeToRun(This,wake) ) 

#define ITaskSettings3_get_NetworkSettings(This,ppNetworkSettings)	\
    ( (This)->lpVtbl -> get_NetworkSettings(This,ppNetworkSettings) ) 

#define ITaskSettings3_put_NetworkSettings(This,pNetworkSettings)	\
    ( (This)->lpVtbl -> put_NetworkSettings(This,pNetworkSettings) ) 


#define ITaskSettings3_get_DisallowStartOnRemoteAppSession(This,pDisallowStart)	\
    ( (This)->lpVtbl -> get_DisallowStartOnRemoteAppSession(This,pDisallowStart) ) 

#define ITaskSettings3_put_DisallowStartOnRemoteAppSession(This,disallowStart)	\
    ( (This)->lpVtbl -> put_DisallowStartOnRemoteAppSession(This,disallowStart) ) 

#define ITaskSettings3_get_UseUnifiedSchedulingEngine(This,pUseUnifiedEngine)	\
    ( (This)->lpVtbl -> get_UseUnifiedSchedulingEngine(This,pUseUnifiedEngine) ) 

#define ITaskSettings3_put_UseUnifiedSchedulingEngine(This,useUnifiedEngine)	\
    ( (This)->lpVtbl -> put_UseUnifiedSchedulingEngine(This,useUnifiedEngine) ) 

#define ITaskSettings3_get_MaintenanceSettings(This,ppMaintenanceSettings)	\
    ( (This)->lpVtbl -> get_MaintenanceSettings(This,ppMaintenanceSettings) ) 

#define ITaskSettings3_put_MaintenanceSettings(This,pMaintenanceSettings)	\
    ( (This)->lpVtbl -> put_MaintenanceSettings(This,pMaintenanceSettings) ) 

#define ITaskSettings3_CreateMaintenanceSettings(This,ppMaintenanceSettings)	\
    ( (This)->lpVtbl -> CreateMaintenanceSettings(This,ppMaintenanceSettings) ) 

#define ITaskSettings3_get_Volatile(This,pVolatile)	\
    ( (This)->lpVtbl -> get_Volatile(This,pVolatile) ) 

#define ITaskSettings3_put_Volatile(This,Volatile)	\
    ( (This)->lpVtbl -> put_Volatile(This,Volatile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskSettings3_INTERFACE_DEFINED__ */


#ifndef __IMaintenanceSettings_INTERFACE_DEFINED__
#define __IMaintenanceSettings_INTERFACE_DEFINED__

/* interface IMaintenanceSettings */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IMaintenanceSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6024FA8-9652-4ADB-A6BF-5CFCD877A7BA")
    IMaintenanceSettings : public IDispatch
    {
    public:
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Period( 
            /* [in] */ BSTR value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Period( 
            /* [retval][out] */ BSTR *target) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Deadline( 
            /* [in] */ BSTR value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Deadline( 
            /* [retval][out] */ BSTR *target) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Exclusive( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Exclusive( 
            /* [retval][out] */ VARIANT_BOOL *target) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMaintenanceSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMaintenanceSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMaintenanceSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMaintenanceSettings * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMaintenanceSettings * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMaintenanceSettings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMaintenanceSettings * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMaintenanceSettings * This,
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
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, put_Period)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Period )( 
            IMaintenanceSettings * This,
            /* [in] */ BSTR value);
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, get_Period)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Period )( 
            IMaintenanceSettings * This,
            /* [retval][out] */ BSTR *target);
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, put_Deadline)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Deadline )( 
            IMaintenanceSettings * This,
            /* [in] */ BSTR value);
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, get_Deadline)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Deadline )( 
            IMaintenanceSettings * This,
            /* [retval][out] */ BSTR *target);
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, put_Exclusive)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Exclusive )( 
            IMaintenanceSettings * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IMaintenanceSettings, get_Exclusive)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Exclusive )( 
            IMaintenanceSettings * This,
            /* [retval][out] */ VARIANT_BOOL *target);
        
        END_INTERFACE
    } IMaintenanceSettingsVtbl;

    interface IMaintenanceSettings
    {
        CONST_VTBL struct IMaintenanceSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMaintenanceSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMaintenanceSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMaintenanceSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMaintenanceSettings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMaintenanceSettings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMaintenanceSettings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMaintenanceSettings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMaintenanceSettings_put_Period(This,value)	\
    ( (This)->lpVtbl -> put_Period(This,value) ) 

#define IMaintenanceSettings_get_Period(This,target)	\
    ( (This)->lpVtbl -> get_Period(This,target) ) 

#define IMaintenanceSettings_put_Deadline(This,value)	\
    ( (This)->lpVtbl -> put_Deadline(This,value) ) 

#define IMaintenanceSettings_get_Deadline(This,target)	\
    ( (This)->lpVtbl -> get_Deadline(This,target) ) 

#define IMaintenanceSettings_put_Exclusive(This,value)	\
    ( (This)->lpVtbl -> put_Exclusive(This,value) ) 

#define IMaintenanceSettings_get_Exclusive(This,target)	\
    ( (This)->lpVtbl -> get_Exclusive(This,target) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMaintenanceSettings_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_TaskScheduler;

#ifdef __cplusplus

class DECLSPEC_UUID("0f87369f-a4e5-4cfc-bd3e-73e6154572dd")
TaskScheduler;
#endif

EXTERN_C const CLSID CLSID_TaskHandlerPS;

#ifdef __cplusplus

class DECLSPEC_UUID("f2a69db7-da2c-4352-9066-86fee6dacac9")
TaskHandlerPS;
#endif

EXTERN_C const CLSID CLSID_TaskHandlerStatusPS;

#ifdef __cplusplus

class DECLSPEC_UUID("9f15266d-d7ba-48f0-93c1-e6895f6fe5ac")
TaskHandlerStatusPS;
#endif
#endif /* __TaskScheduler_LIBRARY_DEFINED__ */

#ifndef __IRegisteredTaskCollection_INTERFACE_DEFINED__
#define __IRegisteredTaskCollection_INTERFACE_DEFINED__

/* interface IRegisteredTaskCollection */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IRegisteredTaskCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86627eb4-42a7-41e4-a4d9-ac33a72f2d52")
    IRegisteredTaskCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppRegisteredTask) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegisteredTaskCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRegisteredTaskCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRegisteredTaskCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRegisteredTaskCollection * This,
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
        
        DECLSPEC_XFGVIRT(IRegisteredTaskCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(IRegisteredTaskCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppRegisteredTask);
        
        DECLSPEC_XFGVIRT(IRegisteredTaskCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IRegisteredTaskCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        END_INTERFACE
    } IRegisteredTaskCollectionVtbl;

    interface IRegisteredTaskCollection
    {
        CONST_VTBL struct IRegisteredTaskCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegisteredTaskCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegisteredTaskCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegisteredTaskCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegisteredTaskCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRegisteredTaskCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRegisteredTaskCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRegisteredTaskCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRegisteredTaskCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IRegisteredTaskCollection_get_Item(This,index,ppRegisteredTask)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppRegisteredTask) ) 

#define IRegisteredTaskCollection_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegisteredTaskCollection_INTERFACE_DEFINED__ */


#ifndef __ITaskFolder_INTERFACE_DEFINED__
#define __ITaskFolder_INTERFACE_DEFINED__

/* interface ITaskFolder */
/* [helpstring][nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITaskFolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8cfac062-a080-4c15-9a88-aa7c2af80dfc")
    ITaskFolder : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFolder( 
            __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFolders( 
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolderCollection **ppFolders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateFolder( 
            /* [in] */ __RPC__in BSTR subFolderName,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteFolder( 
            __RPC__in BSTR subFolderName,
            /* [in] */ LONG flags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTask( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTasks( 
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTaskCollection **ppTasks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteTask( 
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ LONG flags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterTask( 
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in BSTR xmlText,
            /* [in] */ LONG flags,
            /* [in] */ VARIANT userId,
            /* [in] */ VARIANT password,
            /* [in] */ TASK_LOGON_TYPE logonType,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterTaskDefinition( 
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in_opt ITaskDefinition *pDefinition,
            /* [in] */ LONG flags,
            /* [in] */ VARIANT userId,
            /* [in] */ VARIANT password,
            /* [in] */ TASK_LOGON_TYPE logonType,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityDescriptor( 
            LONG securityInformation,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSddl) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityDescriptor( 
            /* [in] */ __RPC__in BSTR sddl,
            /* [in] */ LONG flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITaskFolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITaskFolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITaskFolder * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITaskFolder * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITaskFolder * This,
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
        
        DECLSPEC_XFGVIRT(ITaskFolder, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITaskFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITaskFolder, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in ITaskFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPath);
        
        DECLSPEC_XFGVIRT(ITaskFolder, GetFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFolder )( 
            __RPC__in ITaskFolder * This,
            __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder);
        
        DECLSPEC_XFGVIRT(ITaskFolder, GetFolders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFolders )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolderCollection **ppFolders);
        
        DECLSPEC_XFGVIRT(ITaskFolder, CreateFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFolder )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR subFolderName,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt ITaskFolder **ppFolder);
        
        DECLSPEC_XFGVIRT(ITaskFolder, DeleteFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteFolder )( 
            __RPC__in ITaskFolder * This,
            __RPC__in BSTR subFolderName,
            /* [in] */ LONG flags);
        
        DECLSPEC_XFGVIRT(ITaskFolder, GetTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTask )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask);
        
        DECLSPEC_XFGVIRT(ITaskFolder, GetTasks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTasks )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ LONG flags,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTaskCollection **ppTasks);
        
        DECLSPEC_XFGVIRT(ITaskFolder, DeleteTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ LONG flags);
        
        DECLSPEC_XFGVIRT(ITaskFolder, RegisterTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterTask )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in BSTR xmlText,
            /* [in] */ LONG flags,
            /* [in] */ VARIANT userId,
            /* [in] */ VARIANT password,
            /* [in] */ TASK_LOGON_TYPE logonType,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask);
        
        DECLSPEC_XFGVIRT(ITaskFolder, RegisterTaskDefinition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterTaskDefinition )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in_opt ITaskDefinition *pDefinition,
            /* [in] */ LONG flags,
            /* [in] */ VARIANT userId,
            /* [in] */ VARIANT password,
            /* [in] */ TASK_LOGON_TYPE logonType,
            /* [optional][in] */ VARIANT sddl,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredTask **ppTask);
        
        DECLSPEC_XFGVIRT(ITaskFolder, GetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in ITaskFolder * This,
            LONG securityInformation,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSddl);
        
        DECLSPEC_XFGVIRT(ITaskFolder, SetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in ITaskFolder * This,
            /* [in] */ __RPC__in BSTR sddl,
            /* [in] */ LONG flags);
        
        END_INTERFACE
    } ITaskFolderVtbl;

    interface ITaskFolder
    {
        CONST_VTBL struct ITaskFolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITaskFolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITaskFolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITaskFolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITaskFolder_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITaskFolder_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITaskFolder_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITaskFolder_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITaskFolder_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITaskFolder_get_Path(This,pPath)	\
    ( (This)->lpVtbl -> get_Path(This,pPath) ) 

#define ITaskFolder_GetFolder(This,path,ppFolder)	\
    ( (This)->lpVtbl -> GetFolder(This,path,ppFolder) ) 

#define ITaskFolder_GetFolders(This,flags,ppFolders)	\
    ( (This)->lpVtbl -> GetFolders(This,flags,ppFolders) ) 

#define ITaskFolder_CreateFolder(This,subFolderName,sddl,ppFolder)	\
    ( (This)->lpVtbl -> CreateFolder(This,subFolderName,sddl,ppFolder) ) 

#define ITaskFolder_DeleteFolder(This,subFolderName,flags)	\
    ( (This)->lpVtbl -> DeleteFolder(This,subFolderName,flags) ) 

#define ITaskFolder_GetTask(This,path,ppTask)	\
    ( (This)->lpVtbl -> GetTask(This,path,ppTask) ) 

#define ITaskFolder_GetTasks(This,flags,ppTasks)	\
    ( (This)->lpVtbl -> GetTasks(This,flags,ppTasks) ) 

#define ITaskFolder_DeleteTask(This,name,flags)	\
    ( (This)->lpVtbl -> DeleteTask(This,name,flags) ) 

#define ITaskFolder_RegisterTask(This,path,xmlText,flags,userId,password,logonType,sddl,ppTask)	\
    ( (This)->lpVtbl -> RegisterTask(This,path,xmlText,flags,userId,password,logonType,sddl,ppTask) ) 

#define ITaskFolder_RegisterTaskDefinition(This,path,pDefinition,flags,userId,password,logonType,sddl,ppTask)	\
    ( (This)->lpVtbl -> RegisterTaskDefinition(This,path,pDefinition,flags,userId,password,logonType,sddl,ppTask) ) 

#define ITaskFolder_GetSecurityDescriptor(This,securityInformation,pSddl)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,securityInformation,pSddl) ) 

#define ITaskFolder_SetSecurityDescriptor(This,sddl,flags)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,sddl,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITaskFolder_INTERFACE_DEFINED__ */


#ifndef __IIdleSettings_INTERFACE_DEFINED__
#define __IIdleSettings_INTERFACE_DEFINED__

/* interface IIdleSettings */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IIdleSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("84594461-0053-4342-A8FD-088FABF11F32")
    IIdleSettings : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IdleDuration( 
            /* [retval][out] */ BSTR *pDelay) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_IdleDuration( 
            /* [in] */ BSTR delay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WaitTimeout( 
            /* [retval][out] */ BSTR *pTimeout) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_WaitTimeout( 
            /* [in] */ BSTR timeout) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StopOnIdleEnd( 
            /* [retval][out] */ VARIANT_BOOL *pStop) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StopOnIdleEnd( 
            /* [in] */ VARIANT_BOOL stop) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RestartOnIdle( 
            /* [retval][out] */ VARIANT_BOOL *pRestart) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RestartOnIdle( 
            /* [in] */ VARIANT_BOOL restart) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdleSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIdleSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIdleSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIdleSettings * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IIdleSettings * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IIdleSettings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IIdleSettings * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIdleSettings * This,
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
        
        DECLSPEC_XFGVIRT(IIdleSettings, get_IdleDuration)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IdleDuration )( 
            IIdleSettings * This,
            /* [retval][out] */ BSTR *pDelay);
        
        DECLSPEC_XFGVIRT(IIdleSettings, put_IdleDuration)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IdleDuration )( 
            IIdleSettings * This,
            /* [in] */ BSTR delay);
        
        DECLSPEC_XFGVIRT(IIdleSettings, get_WaitTimeout)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WaitTimeout )( 
            IIdleSettings * This,
            /* [retval][out] */ BSTR *pTimeout);
        
        DECLSPEC_XFGVIRT(IIdleSettings, put_WaitTimeout)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_WaitTimeout )( 
            IIdleSettings * This,
            /* [in] */ BSTR timeout);
        
        DECLSPEC_XFGVIRT(IIdleSettings, get_StopOnIdleEnd)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StopOnIdleEnd )( 
            IIdleSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pStop);
        
        DECLSPEC_XFGVIRT(IIdleSettings, put_StopOnIdleEnd)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StopOnIdleEnd )( 
            IIdleSettings * This,
            /* [in] */ VARIANT_BOOL stop);
        
        DECLSPEC_XFGVIRT(IIdleSettings, get_RestartOnIdle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RestartOnIdle )( 
            IIdleSettings * This,
            /* [retval][out] */ VARIANT_BOOL *pRestart);
        
        DECLSPEC_XFGVIRT(IIdleSettings, put_RestartOnIdle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RestartOnIdle )( 
            IIdleSettings * This,
            /* [in] */ VARIANT_BOOL restart);
        
        END_INTERFACE
    } IIdleSettingsVtbl;

    interface IIdleSettings
    {
        CONST_VTBL struct IIdleSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdleSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdleSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdleSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdleSettings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIdleSettings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIdleSettings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIdleSettings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIdleSettings_get_IdleDuration(This,pDelay)	\
    ( (This)->lpVtbl -> get_IdleDuration(This,pDelay) ) 

#define IIdleSettings_put_IdleDuration(This,delay)	\
    ( (This)->lpVtbl -> put_IdleDuration(This,delay) ) 

#define IIdleSettings_get_WaitTimeout(This,pTimeout)	\
    ( (This)->lpVtbl -> get_WaitTimeout(This,pTimeout) ) 

#define IIdleSettings_put_WaitTimeout(This,timeout)	\
    ( (This)->lpVtbl -> put_WaitTimeout(This,timeout) ) 

#define IIdleSettings_get_StopOnIdleEnd(This,pStop)	\
    ( (This)->lpVtbl -> get_StopOnIdleEnd(This,pStop) ) 

#define IIdleSettings_put_StopOnIdleEnd(This,stop)	\
    ( (This)->lpVtbl -> put_StopOnIdleEnd(This,stop) ) 

#define IIdleSettings_get_RestartOnIdle(This,pRestart)	\
    ( (This)->lpVtbl -> get_RestartOnIdle(This,pRestart) ) 

#define IIdleSettings_put_RestartOnIdle(This,restart)	\
    ( (This)->lpVtbl -> put_RestartOnIdle(This,restart) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdleSettings_INTERFACE_DEFINED__ */


#ifndef __INetworkSettings_INTERFACE_DEFINED__
#define __INetworkSettings_INTERFACE_DEFINED__

/* interface INetworkSettings */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_INetworkSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7DEA84-C30B-4245-80B6-00E9F646F1B4")
    INetworkSettings : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *pName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ BSTR name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ BSTR *pId) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Id( 
            /* [in] */ BSTR id) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetworkSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetworkSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetworkSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetworkSettings * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            INetworkSettings * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            INetworkSettings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            INetworkSettings * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INetworkSettings * This,
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
        
        DECLSPEC_XFGVIRT(INetworkSettings, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            INetworkSettings * This,
            /* [retval][out] */ BSTR *pName);
        
        DECLSPEC_XFGVIRT(INetworkSettings, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            INetworkSettings * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(INetworkSettings, get_Id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            INetworkSettings * This,
            /* [retval][out] */ BSTR *pId);
        
        DECLSPEC_XFGVIRT(INetworkSettings, put_Id)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Id )( 
            INetworkSettings * This,
            /* [in] */ BSTR id);
        
        END_INTERFACE
    } INetworkSettingsVtbl;

    interface INetworkSettings
    {
        CONST_VTBL struct INetworkSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetworkSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetworkSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetworkSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetworkSettings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INetworkSettings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INetworkSettings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INetworkSettings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INetworkSettings_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define INetworkSettings_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define INetworkSettings_get_Id(This,pId)	\
    ( (This)->lpVtbl -> get_Id(This,pId) ) 

#define INetworkSettings_put_Id(This,id)	\
    ( (This)->lpVtbl -> put_Id(This,id) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetworkSettings_INTERFACE_DEFINED__ */


#ifndef __IRepetitionPattern_INTERFACE_DEFINED__
#define __IRepetitionPattern_INTERFACE_DEFINED__

/* interface IRepetitionPattern */
/* [helpstring][nonextensible][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IRepetitionPattern;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FB9ACF1-26BE-400e-85B5-294B9C75DFD6")
    IRepetitionPattern : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Interval( 
            /* [retval][out] */ BSTR *pInterval) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Interval( 
            /* [in] */ BSTR interval) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Duration( 
            /* [retval][out] */ BSTR *pDuration) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Duration( 
            /* [in] */ BSTR duration) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StopAtDurationEnd( 
            /* [retval][out] */ VARIANT_BOOL *pStop) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StopAtDurationEnd( 
            /* [in] */ VARIANT_BOOL stop) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRepetitionPatternVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRepetitionPattern * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRepetitionPattern * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRepetitionPattern * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IRepetitionPattern * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IRepetitionPattern * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IRepetitionPattern * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRepetitionPattern * This,
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
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, get_Interval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Interval )( 
            IRepetitionPattern * This,
            /* [retval][out] */ BSTR *pInterval);
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, put_Interval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Interval )( 
            IRepetitionPattern * This,
            /* [in] */ BSTR interval);
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, get_Duration)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Duration )( 
            IRepetitionPattern * This,
            /* [retval][out] */ BSTR *pDuration);
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, put_Duration)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Duration )( 
            IRepetitionPattern * This,
            /* [in] */ BSTR duration);
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, get_StopAtDurationEnd)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StopAtDurationEnd )( 
            IRepetitionPattern * This,
            /* [retval][out] */ VARIANT_BOOL *pStop);
        
        DECLSPEC_XFGVIRT(IRepetitionPattern, put_StopAtDurationEnd)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StopAtDurationEnd )( 
            IRepetitionPattern * This,
            /* [in] */ VARIANT_BOOL stop);
        
        END_INTERFACE
    } IRepetitionPatternVtbl;

    interface IRepetitionPattern
    {
        CONST_VTBL struct IRepetitionPatternVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRepetitionPattern_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRepetitionPattern_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRepetitionPattern_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRepetitionPattern_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRepetitionPattern_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRepetitionPattern_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRepetitionPattern_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRepetitionPattern_get_Interval(This,pInterval)	\
    ( (This)->lpVtbl -> get_Interval(This,pInterval) ) 

#define IRepetitionPattern_put_Interval(This,interval)	\
    ( (This)->lpVtbl -> put_Interval(This,interval) ) 

#define IRepetitionPattern_get_Duration(This,pDuration)	\
    ( (This)->lpVtbl -> get_Duration(This,pDuration) ) 

#define IRepetitionPattern_put_Duration(This,duration)	\
    ( (This)->lpVtbl -> put_Duration(This,duration) ) 

#define IRepetitionPattern_get_StopAtDurationEnd(This,pStop)	\
    ( (This)->lpVtbl -> get_StopAtDurationEnd(This,pStop) ) 

#define IRepetitionPattern_put_StopAtDurationEnd(This,stop)	\
    ( (This)->lpVtbl -> put_StopAtDurationEnd(This,stop) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRepetitionPattern_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_taskschd_0000_0044 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_taskschd_0000_0044_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_taskschd_0000_0044_v0_0_s_ifspec;

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


