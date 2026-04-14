

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

#ifndef __autosvcs_h__
#define __autosvcs_h__

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

#ifndef __ISecurityIdentityColl_FWD_DEFINED__
#define __ISecurityIdentityColl_FWD_DEFINED__
typedef interface ISecurityIdentityColl ISecurityIdentityColl;

#endif 	/* __ISecurityIdentityColl_FWD_DEFINED__ */


#ifndef __ISecurityCallersColl_FWD_DEFINED__
#define __ISecurityCallersColl_FWD_DEFINED__
typedef interface ISecurityCallersColl ISecurityCallersColl;

#endif 	/* __ISecurityCallersColl_FWD_DEFINED__ */


#ifndef __ISecurityCallContext_FWD_DEFINED__
#define __ISecurityCallContext_FWD_DEFINED__
typedef interface ISecurityCallContext ISecurityCallContext;

#endif 	/* __ISecurityCallContext_FWD_DEFINED__ */


#ifndef __IGetSecurityCallContext_FWD_DEFINED__
#define __IGetSecurityCallContext_FWD_DEFINED__
typedef interface IGetSecurityCallContext IGetSecurityCallContext;

#endif 	/* __IGetSecurityCallContext_FWD_DEFINED__ */


#ifndef __SecurityProperty_FWD_DEFINED__
#define __SecurityProperty_FWD_DEFINED__
typedef interface SecurityProperty SecurityProperty;

#endif 	/* __SecurityProperty_FWD_DEFINED__ */


#ifndef __ContextInfo_FWD_DEFINED__
#define __ContextInfo_FWD_DEFINED__
typedef interface ContextInfo ContextInfo;

#endif 	/* __ContextInfo_FWD_DEFINED__ */


#ifndef __ContextInfo2_FWD_DEFINED__
#define __ContextInfo2_FWD_DEFINED__
typedef interface ContextInfo2 ContextInfo2;

#endif 	/* __ContextInfo2_FWD_DEFINED__ */


#ifndef __ObjectContext_FWD_DEFINED__
#define __ObjectContext_FWD_DEFINED__
typedef interface ObjectContext ObjectContext;

#endif 	/* __ObjectContext_FWD_DEFINED__ */


#ifndef __ITransactionContextEx_FWD_DEFINED__
#define __ITransactionContextEx_FWD_DEFINED__
typedef interface ITransactionContextEx ITransactionContextEx;

#endif 	/* __ITransactionContextEx_FWD_DEFINED__ */


#ifndef __ITransactionContext_FWD_DEFINED__
#define __ITransactionContext_FWD_DEFINED__
typedef interface ITransactionContext ITransactionContext;

#endif 	/* __ITransactionContext_FWD_DEFINED__ */


#ifndef __ICreateWithTransactionEx_FWD_DEFINED__
#define __ICreateWithTransactionEx_FWD_DEFINED__
typedef interface ICreateWithTransactionEx ICreateWithTransactionEx;

#endif 	/* __ICreateWithTransactionEx_FWD_DEFINED__ */


#ifndef __ICreateWithLocalTransaction_FWD_DEFINED__
#define __ICreateWithLocalTransaction_FWD_DEFINED__
typedef interface ICreateWithLocalTransaction ICreateWithLocalTransaction;

#endif 	/* __ICreateWithLocalTransaction_FWD_DEFINED__ */


#ifndef __ICreateWithTipTransactionEx_FWD_DEFINED__
#define __ICreateWithTipTransactionEx_FWD_DEFINED__
typedef interface ICreateWithTipTransactionEx ICreateWithTipTransactionEx;

#endif 	/* __ICreateWithTipTransactionEx_FWD_DEFINED__ */


#ifndef __IComLTxEvents_FWD_DEFINED__
#define __IComLTxEvents_FWD_DEFINED__
typedef interface IComLTxEvents IComLTxEvents;

#endif 	/* __IComLTxEvents_FWD_DEFINED__ */


#ifndef __IComUserEvent_FWD_DEFINED__
#define __IComUserEvent_FWD_DEFINED__
typedef interface IComUserEvent IComUserEvent;

#endif 	/* __IComUserEvent_FWD_DEFINED__ */


#ifndef __IComThreadEvents_FWD_DEFINED__
#define __IComThreadEvents_FWD_DEFINED__
typedef interface IComThreadEvents IComThreadEvents;

#endif 	/* __IComThreadEvents_FWD_DEFINED__ */


#ifndef __IComAppEvents_FWD_DEFINED__
#define __IComAppEvents_FWD_DEFINED__
typedef interface IComAppEvents IComAppEvents;

#endif 	/* __IComAppEvents_FWD_DEFINED__ */


#ifndef __IComInstanceEvents_FWD_DEFINED__
#define __IComInstanceEvents_FWD_DEFINED__
typedef interface IComInstanceEvents IComInstanceEvents;

#endif 	/* __IComInstanceEvents_FWD_DEFINED__ */


#ifndef __IComTransactionEvents_FWD_DEFINED__
#define __IComTransactionEvents_FWD_DEFINED__
typedef interface IComTransactionEvents IComTransactionEvents;

#endif 	/* __IComTransactionEvents_FWD_DEFINED__ */


#ifndef __IComMethodEvents_FWD_DEFINED__
#define __IComMethodEvents_FWD_DEFINED__
typedef interface IComMethodEvents IComMethodEvents;

#endif 	/* __IComMethodEvents_FWD_DEFINED__ */


#ifndef __IComObjectEvents_FWD_DEFINED__
#define __IComObjectEvents_FWD_DEFINED__
typedef interface IComObjectEvents IComObjectEvents;

#endif 	/* __IComObjectEvents_FWD_DEFINED__ */


#ifndef __IComResourceEvents_FWD_DEFINED__
#define __IComResourceEvents_FWD_DEFINED__
typedef interface IComResourceEvents IComResourceEvents;

#endif 	/* __IComResourceEvents_FWD_DEFINED__ */


#ifndef __IComSecurityEvents_FWD_DEFINED__
#define __IComSecurityEvents_FWD_DEFINED__
typedef interface IComSecurityEvents IComSecurityEvents;

#endif 	/* __IComSecurityEvents_FWD_DEFINED__ */


#ifndef __IComObjectPoolEvents_FWD_DEFINED__
#define __IComObjectPoolEvents_FWD_DEFINED__
typedef interface IComObjectPoolEvents IComObjectPoolEvents;

#endif 	/* __IComObjectPoolEvents_FWD_DEFINED__ */


#ifndef __IComObjectPoolEvents2_FWD_DEFINED__
#define __IComObjectPoolEvents2_FWD_DEFINED__
typedef interface IComObjectPoolEvents2 IComObjectPoolEvents2;

#endif 	/* __IComObjectPoolEvents2_FWD_DEFINED__ */


#ifndef __IComObjectConstructionEvents_FWD_DEFINED__
#define __IComObjectConstructionEvents_FWD_DEFINED__
typedef interface IComObjectConstructionEvents IComObjectConstructionEvents;

#endif 	/* __IComObjectConstructionEvents_FWD_DEFINED__ */


#ifndef __IComActivityEvents_FWD_DEFINED__
#define __IComActivityEvents_FWD_DEFINED__
typedef interface IComActivityEvents IComActivityEvents;

#endif 	/* __IComActivityEvents_FWD_DEFINED__ */


#ifndef __IComIdentityEvents_FWD_DEFINED__
#define __IComIdentityEvents_FWD_DEFINED__
typedef interface IComIdentityEvents IComIdentityEvents;

#endif 	/* __IComIdentityEvents_FWD_DEFINED__ */


#ifndef __IComQCEvents_FWD_DEFINED__
#define __IComQCEvents_FWD_DEFINED__
typedef interface IComQCEvents IComQCEvents;

#endif 	/* __IComQCEvents_FWD_DEFINED__ */


#ifndef __IComExceptionEvents_FWD_DEFINED__
#define __IComExceptionEvents_FWD_DEFINED__
typedef interface IComExceptionEvents IComExceptionEvents;

#endif 	/* __IComExceptionEvents_FWD_DEFINED__ */


#ifndef __ILBEvents_FWD_DEFINED__
#define __ILBEvents_FWD_DEFINED__
typedef interface ILBEvents ILBEvents;

#endif 	/* __ILBEvents_FWD_DEFINED__ */


#ifndef __IComCRMEvents_FWD_DEFINED__
#define __IComCRMEvents_FWD_DEFINED__
typedef interface IComCRMEvents IComCRMEvents;

#endif 	/* __IComCRMEvents_FWD_DEFINED__ */


#ifndef __IComMethod2Events_FWD_DEFINED__
#define __IComMethod2Events_FWD_DEFINED__
typedef interface IComMethod2Events IComMethod2Events;

#endif 	/* __IComMethod2Events_FWD_DEFINED__ */


#ifndef __IComTrackingInfoEvents_FWD_DEFINED__
#define __IComTrackingInfoEvents_FWD_DEFINED__
typedef interface IComTrackingInfoEvents IComTrackingInfoEvents;

#endif 	/* __IComTrackingInfoEvents_FWD_DEFINED__ */


#ifndef __IComTrackingInfoCollection_FWD_DEFINED__
#define __IComTrackingInfoCollection_FWD_DEFINED__
typedef interface IComTrackingInfoCollection IComTrackingInfoCollection;

#endif 	/* __IComTrackingInfoCollection_FWD_DEFINED__ */


#ifndef __IComTrackingInfoObject_FWD_DEFINED__
#define __IComTrackingInfoObject_FWD_DEFINED__
typedef interface IComTrackingInfoObject IComTrackingInfoObject;

#endif 	/* __IComTrackingInfoObject_FWD_DEFINED__ */


#ifndef __IComTrackingInfoProperties_FWD_DEFINED__
#define __IComTrackingInfoProperties_FWD_DEFINED__
typedef interface IComTrackingInfoProperties IComTrackingInfoProperties;

#endif 	/* __IComTrackingInfoProperties_FWD_DEFINED__ */


#ifndef __IComApp2Events_FWD_DEFINED__
#define __IComApp2Events_FWD_DEFINED__
typedef interface IComApp2Events IComApp2Events;

#endif 	/* __IComApp2Events_FWD_DEFINED__ */


#ifndef __IComTransaction2Events_FWD_DEFINED__
#define __IComTransaction2Events_FWD_DEFINED__
typedef interface IComTransaction2Events IComTransaction2Events;

#endif 	/* __IComTransaction2Events_FWD_DEFINED__ */


#ifndef __IComInstance2Events_FWD_DEFINED__
#define __IComInstance2Events_FWD_DEFINED__
typedef interface IComInstance2Events IComInstance2Events;

#endif 	/* __IComInstance2Events_FWD_DEFINED__ */


#ifndef __IComObjectPool2Events_FWD_DEFINED__
#define __IComObjectPool2Events_FWD_DEFINED__
typedef interface IComObjectPool2Events IComObjectPool2Events;

#endif 	/* __IComObjectPool2Events_FWD_DEFINED__ */


#ifndef __IComObjectConstruction2Events_FWD_DEFINED__
#define __IComObjectConstruction2Events_FWD_DEFINED__
typedef interface IComObjectConstruction2Events IComObjectConstruction2Events;

#endif 	/* __IComObjectConstruction2Events_FWD_DEFINED__ */


#ifndef __ISystemAppEventData_FWD_DEFINED__
#define __ISystemAppEventData_FWD_DEFINED__
typedef interface ISystemAppEventData ISystemAppEventData;

#endif 	/* __ISystemAppEventData_FWD_DEFINED__ */


#ifndef __IMtsEvents_FWD_DEFINED__
#define __IMtsEvents_FWD_DEFINED__
typedef interface IMtsEvents IMtsEvents;

#endif 	/* __IMtsEvents_FWD_DEFINED__ */


#ifndef __IMtsEventInfo_FWD_DEFINED__
#define __IMtsEventInfo_FWD_DEFINED__
typedef interface IMtsEventInfo IMtsEventInfo;

#endif 	/* __IMtsEventInfo_FWD_DEFINED__ */


#ifndef __IMTSLocator_FWD_DEFINED__
#define __IMTSLocator_FWD_DEFINED__
typedef interface IMTSLocator IMTSLocator;

#endif 	/* __IMTSLocator_FWD_DEFINED__ */


#ifndef __IMtsGrp_FWD_DEFINED__
#define __IMtsGrp_FWD_DEFINED__
typedef interface IMtsGrp IMtsGrp;

#endif 	/* __IMtsGrp_FWD_DEFINED__ */


#ifndef __IMessageMover_FWD_DEFINED__
#define __IMessageMover_FWD_DEFINED__
typedef interface IMessageMover IMessageMover;

#endif 	/* __IMessageMover_FWD_DEFINED__ */


#ifndef __IEventServerTrace_FWD_DEFINED__
#define __IEventServerTrace_FWD_DEFINED__
typedef interface IEventServerTrace IEventServerTrace;

#endif 	/* __IEventServerTrace_FWD_DEFINED__ */


#ifndef __IGetAppTrackerData_FWD_DEFINED__
#define __IGetAppTrackerData_FWD_DEFINED__
typedef interface IGetAppTrackerData IGetAppTrackerData;

#endif 	/* __IGetAppTrackerData_FWD_DEFINED__ */


#ifndef __IDispenserManager_FWD_DEFINED__
#define __IDispenserManager_FWD_DEFINED__
typedef interface IDispenserManager IDispenserManager;

#endif 	/* __IDispenserManager_FWD_DEFINED__ */


#ifndef __IHolder_FWD_DEFINED__
#define __IHolder_FWD_DEFINED__
typedef interface IHolder IHolder;

#endif 	/* __IHolder_FWD_DEFINED__ */


#ifndef __IDispenserDriver_FWD_DEFINED__
#define __IDispenserDriver_FWD_DEFINED__
typedef interface IDispenserDriver IDispenserDriver;

#endif 	/* __IDispenserDriver_FWD_DEFINED__ */


#ifndef __ITransactionProxy_FWD_DEFINED__
#define __ITransactionProxy_FWD_DEFINED__
typedef interface ITransactionProxy ITransactionProxy;

#endif 	/* __ITransactionProxy_FWD_DEFINED__ */


#ifndef __IContextSecurityPerimeter_FWD_DEFINED__
#define __IContextSecurityPerimeter_FWD_DEFINED__
typedef interface IContextSecurityPerimeter IContextSecurityPerimeter;

#endif 	/* __IContextSecurityPerimeter_FWD_DEFINED__ */


#ifndef __ITxProxyHolder_FWD_DEFINED__
#define __ITxProxyHolder_FWD_DEFINED__
typedef interface ITxProxyHolder ITxProxyHolder;

#endif 	/* __ITxProxyHolder_FWD_DEFINED__ */


#ifndef __IObjectContext_FWD_DEFINED__
#define __IObjectContext_FWD_DEFINED__
typedef interface IObjectContext IObjectContext;

#endif 	/* __IObjectContext_FWD_DEFINED__ */


#ifndef __IObjectControl_FWD_DEFINED__
#define __IObjectControl_FWD_DEFINED__
typedef interface IObjectControl IObjectControl;

#endif 	/* __IObjectControl_FWD_DEFINED__ */


#ifndef __IEnumNames_FWD_DEFINED__
#define __IEnumNames_FWD_DEFINED__
typedef interface IEnumNames IEnumNames;

#endif 	/* __IEnumNames_FWD_DEFINED__ */


#ifndef __ISecurityProperty_FWD_DEFINED__
#define __ISecurityProperty_FWD_DEFINED__
typedef interface ISecurityProperty ISecurityProperty;

#endif 	/* __ISecurityProperty_FWD_DEFINED__ */


#ifndef __ObjectControl_FWD_DEFINED__
#define __ObjectControl_FWD_DEFINED__
typedef interface ObjectControl ObjectControl;

#endif 	/* __ObjectControl_FWD_DEFINED__ */


#ifndef __ISharedProperty_FWD_DEFINED__
#define __ISharedProperty_FWD_DEFINED__
typedef interface ISharedProperty ISharedProperty;

#endif 	/* __ISharedProperty_FWD_DEFINED__ */


#ifndef __ISharedPropertyGroup_FWD_DEFINED__
#define __ISharedPropertyGroup_FWD_DEFINED__
typedef interface ISharedPropertyGroup ISharedPropertyGroup;

#endif 	/* __ISharedPropertyGroup_FWD_DEFINED__ */


#ifndef __ISharedPropertyGroupManager_FWD_DEFINED__
#define __ISharedPropertyGroupManager_FWD_DEFINED__
typedef interface ISharedPropertyGroupManager ISharedPropertyGroupManager;

#endif 	/* __ISharedPropertyGroupManager_FWD_DEFINED__ */


#ifndef __IObjectConstruct_FWD_DEFINED__
#define __IObjectConstruct_FWD_DEFINED__
typedef interface IObjectConstruct IObjectConstruct;

#endif 	/* __IObjectConstruct_FWD_DEFINED__ */


#ifndef __IObjectConstructString_FWD_DEFINED__
#define __IObjectConstructString_FWD_DEFINED__
typedef interface IObjectConstructString IObjectConstructString;

#endif 	/* __IObjectConstructString_FWD_DEFINED__ */


#ifndef __IObjectContextActivity_FWD_DEFINED__
#define __IObjectContextActivity_FWD_DEFINED__
typedef interface IObjectContextActivity IObjectContextActivity;

#endif 	/* __IObjectContextActivity_FWD_DEFINED__ */


#ifndef __IObjectContextInfo_FWD_DEFINED__
#define __IObjectContextInfo_FWD_DEFINED__
typedef interface IObjectContextInfo IObjectContextInfo;

#endif 	/* __IObjectContextInfo_FWD_DEFINED__ */


#ifndef __IObjectContextInfo2_FWD_DEFINED__
#define __IObjectContextInfo2_FWD_DEFINED__
typedef interface IObjectContextInfo2 IObjectContextInfo2;

#endif 	/* __IObjectContextInfo2_FWD_DEFINED__ */


#ifndef __ITransactionStatus_FWD_DEFINED__
#define __ITransactionStatus_FWD_DEFINED__
typedef interface ITransactionStatus ITransactionStatus;

#endif 	/* __ITransactionStatus_FWD_DEFINED__ */


#ifndef __IObjectContextTip_FWD_DEFINED__
#define __IObjectContextTip_FWD_DEFINED__
typedef interface IObjectContextTip IObjectContextTip;

#endif 	/* __IObjectContextTip_FWD_DEFINED__ */


#ifndef __IPlaybackControl_FWD_DEFINED__
#define __IPlaybackControl_FWD_DEFINED__
typedef interface IPlaybackControl IPlaybackControl;

#endif 	/* __IPlaybackControl_FWD_DEFINED__ */


#ifndef __IGetContextProperties_FWD_DEFINED__
#define __IGetContextProperties_FWD_DEFINED__
typedef interface IGetContextProperties IGetContextProperties;

#endif 	/* __IGetContextProperties_FWD_DEFINED__ */


#ifndef __IContextState_FWD_DEFINED__
#define __IContextState_FWD_DEFINED__
typedef interface IContextState IContextState;

#endif 	/* __IContextState_FWD_DEFINED__ */


#ifndef __IPoolManager_FWD_DEFINED__
#define __IPoolManager_FWD_DEFINED__
typedef interface IPoolManager IPoolManager;

#endif 	/* __IPoolManager_FWD_DEFINED__ */


#ifndef __ISelectCOMLBServer_FWD_DEFINED__
#define __ISelectCOMLBServer_FWD_DEFINED__
typedef interface ISelectCOMLBServer ISelectCOMLBServer;

#endif 	/* __ISelectCOMLBServer_FWD_DEFINED__ */


#ifndef __ICOMLBArguments_FWD_DEFINED__
#define __ICOMLBArguments_FWD_DEFINED__
typedef interface ICOMLBArguments ICOMLBArguments;

#endif 	/* __ICOMLBArguments_FWD_DEFINED__ */


#ifndef __ICrmLogControl_FWD_DEFINED__
#define __ICrmLogControl_FWD_DEFINED__
typedef interface ICrmLogControl ICrmLogControl;

#endif 	/* __ICrmLogControl_FWD_DEFINED__ */


#ifndef __ICrmCompensatorVariants_FWD_DEFINED__
#define __ICrmCompensatorVariants_FWD_DEFINED__
typedef interface ICrmCompensatorVariants ICrmCompensatorVariants;

#endif 	/* __ICrmCompensatorVariants_FWD_DEFINED__ */


#ifndef __ICrmCompensator_FWD_DEFINED__
#define __ICrmCompensator_FWD_DEFINED__
typedef interface ICrmCompensator ICrmCompensator;

#endif 	/* __ICrmCompensator_FWD_DEFINED__ */


#ifndef __ICrmMonitorLogRecords_FWD_DEFINED__
#define __ICrmMonitorLogRecords_FWD_DEFINED__
typedef interface ICrmMonitorLogRecords ICrmMonitorLogRecords;

#endif 	/* __ICrmMonitorLogRecords_FWD_DEFINED__ */


#ifndef __ICrmMonitorClerks_FWD_DEFINED__
#define __ICrmMonitorClerks_FWD_DEFINED__
typedef interface ICrmMonitorClerks ICrmMonitorClerks;

#endif 	/* __ICrmMonitorClerks_FWD_DEFINED__ */


#ifndef __ICrmMonitor_FWD_DEFINED__
#define __ICrmMonitor_FWD_DEFINED__
typedef interface ICrmMonitor ICrmMonitor;

#endif 	/* __ICrmMonitor_FWD_DEFINED__ */


#ifndef __ICrmFormatLogRecords_FWD_DEFINED__
#define __ICrmFormatLogRecords_FWD_DEFINED__
typedef interface ICrmFormatLogRecords ICrmFormatLogRecords;

#endif 	/* __ICrmFormatLogRecords_FWD_DEFINED__ */


#ifndef __IServiceIISIntrinsicsConfig_FWD_DEFINED__
#define __IServiceIISIntrinsicsConfig_FWD_DEFINED__
typedef interface IServiceIISIntrinsicsConfig IServiceIISIntrinsicsConfig;

#endif 	/* __IServiceIISIntrinsicsConfig_FWD_DEFINED__ */


#ifndef __IServiceComTIIntrinsicsConfig_FWD_DEFINED__
#define __IServiceComTIIntrinsicsConfig_FWD_DEFINED__
typedef interface IServiceComTIIntrinsicsConfig IServiceComTIIntrinsicsConfig;

#endif 	/* __IServiceComTIIntrinsicsConfig_FWD_DEFINED__ */


#ifndef __IServiceSxsConfig_FWD_DEFINED__
#define __IServiceSxsConfig_FWD_DEFINED__
typedef interface IServiceSxsConfig IServiceSxsConfig;

#endif 	/* __IServiceSxsConfig_FWD_DEFINED__ */


#ifndef __ICheckSxsConfig_FWD_DEFINED__
#define __ICheckSxsConfig_FWD_DEFINED__
typedef interface ICheckSxsConfig ICheckSxsConfig;

#endif 	/* __ICheckSxsConfig_FWD_DEFINED__ */


#ifndef __IServiceInheritanceConfig_FWD_DEFINED__
#define __IServiceInheritanceConfig_FWD_DEFINED__
typedef interface IServiceInheritanceConfig IServiceInheritanceConfig;

#endif 	/* __IServiceInheritanceConfig_FWD_DEFINED__ */


#ifndef __IServiceThreadPoolConfig_FWD_DEFINED__
#define __IServiceThreadPoolConfig_FWD_DEFINED__
typedef interface IServiceThreadPoolConfig IServiceThreadPoolConfig;

#endif 	/* __IServiceThreadPoolConfig_FWD_DEFINED__ */


#ifndef __IServiceTransactionConfigBase_FWD_DEFINED__
#define __IServiceTransactionConfigBase_FWD_DEFINED__
typedef interface IServiceTransactionConfigBase IServiceTransactionConfigBase;

#endif 	/* __IServiceTransactionConfigBase_FWD_DEFINED__ */


#ifndef __IServiceTransactionConfig_FWD_DEFINED__
#define __IServiceTransactionConfig_FWD_DEFINED__
typedef interface IServiceTransactionConfig IServiceTransactionConfig;

#endif 	/* __IServiceTransactionConfig_FWD_DEFINED__ */


#ifndef __IServiceSysTxnConfig_FWD_DEFINED__
#define __IServiceSysTxnConfig_FWD_DEFINED__
typedef interface IServiceSysTxnConfig IServiceSysTxnConfig;

#endif 	/* __IServiceSysTxnConfig_FWD_DEFINED__ */


#ifndef __IServiceSynchronizationConfig_FWD_DEFINED__
#define __IServiceSynchronizationConfig_FWD_DEFINED__
typedef interface IServiceSynchronizationConfig IServiceSynchronizationConfig;

#endif 	/* __IServiceSynchronizationConfig_FWD_DEFINED__ */


#ifndef __IServiceTrackerConfig_FWD_DEFINED__
#define __IServiceTrackerConfig_FWD_DEFINED__
typedef interface IServiceTrackerConfig IServiceTrackerConfig;

#endif 	/* __IServiceTrackerConfig_FWD_DEFINED__ */


#ifndef __IServicePartitionConfig_FWD_DEFINED__
#define __IServicePartitionConfig_FWD_DEFINED__
typedef interface IServicePartitionConfig IServicePartitionConfig;

#endif 	/* __IServicePartitionConfig_FWD_DEFINED__ */


#ifndef __IServiceCall_FWD_DEFINED__
#define __IServiceCall_FWD_DEFINED__
typedef interface IServiceCall IServiceCall;

#endif 	/* __IServiceCall_FWD_DEFINED__ */


#ifndef __IAsyncErrorNotify_FWD_DEFINED__
#define __IAsyncErrorNotify_FWD_DEFINED__
typedef interface IAsyncErrorNotify IAsyncErrorNotify;

#endif 	/* __IAsyncErrorNotify_FWD_DEFINED__ */


#ifndef __IServiceActivity_FWD_DEFINED__
#define __IServiceActivity_FWD_DEFINED__
typedef interface IServiceActivity IServiceActivity;

#endif 	/* __IServiceActivity_FWD_DEFINED__ */


#ifndef __IThreadPoolKnobs_FWD_DEFINED__
#define __IThreadPoolKnobs_FWD_DEFINED__
typedef interface IThreadPoolKnobs IThreadPoolKnobs;

#endif 	/* __IThreadPoolKnobs_FWD_DEFINED__ */


#ifndef __IComStaThreadPoolKnobs_FWD_DEFINED__
#define __IComStaThreadPoolKnobs_FWD_DEFINED__
typedef interface IComStaThreadPoolKnobs IComStaThreadPoolKnobs;

#endif 	/* __IComStaThreadPoolKnobs_FWD_DEFINED__ */


#ifndef __IComMtaThreadPoolKnobs_FWD_DEFINED__
#define __IComMtaThreadPoolKnobs_FWD_DEFINED__
typedef interface IComMtaThreadPoolKnobs IComMtaThreadPoolKnobs;

#endif 	/* __IComMtaThreadPoolKnobs_FWD_DEFINED__ */


#ifndef __IComStaThreadPoolKnobs2_FWD_DEFINED__
#define __IComStaThreadPoolKnobs2_FWD_DEFINED__
typedef interface IComStaThreadPoolKnobs2 IComStaThreadPoolKnobs2;

#endif 	/* __IComStaThreadPoolKnobs2_FWD_DEFINED__ */


#ifndef __IProcessInitializer_FWD_DEFINED__
#define __IProcessInitializer_FWD_DEFINED__
typedef interface IProcessInitializer IProcessInitializer;

#endif 	/* __IProcessInitializer_FWD_DEFINED__ */


#ifndef __IServicePoolConfig_FWD_DEFINED__
#define __IServicePoolConfig_FWD_DEFINED__
typedef interface IServicePoolConfig IServicePoolConfig;

#endif 	/* __IServicePoolConfig_FWD_DEFINED__ */


#ifndef __IServicePool_FWD_DEFINED__
#define __IServicePool_FWD_DEFINED__
typedef interface IServicePool IServicePool;

#endif 	/* __IServicePool_FWD_DEFINED__ */


#ifndef __IManagedPooledObj_FWD_DEFINED__
#define __IManagedPooledObj_FWD_DEFINED__
typedef interface IManagedPooledObj IManagedPooledObj;

#endif 	/* __IManagedPooledObj_FWD_DEFINED__ */


#ifndef __IManagedPoolAction_FWD_DEFINED__
#define __IManagedPoolAction_FWD_DEFINED__
typedef interface IManagedPoolAction IManagedPoolAction;

#endif 	/* __IManagedPoolAction_FWD_DEFINED__ */


#ifndef __IManagedObjectInfo_FWD_DEFINED__
#define __IManagedObjectInfo_FWD_DEFINED__
typedef interface IManagedObjectInfo IManagedObjectInfo;

#endif 	/* __IManagedObjectInfo_FWD_DEFINED__ */


#ifndef __IAppDomainHelper_FWD_DEFINED__
#define __IAppDomainHelper_FWD_DEFINED__
typedef interface IAppDomainHelper IAppDomainHelper;

#endif 	/* __IAppDomainHelper_FWD_DEFINED__ */


#ifndef __IAssemblyLocator_FWD_DEFINED__
#define __IAssemblyLocator_FWD_DEFINED__
typedef interface IAssemblyLocator IAssemblyLocator;

#endif 	/* __IAssemblyLocator_FWD_DEFINED__ */


#ifndef __IManagedActivationEvents_FWD_DEFINED__
#define __IManagedActivationEvents_FWD_DEFINED__
typedef interface IManagedActivationEvents IManagedActivationEvents;

#endif 	/* __IManagedActivationEvents_FWD_DEFINED__ */


#ifndef __ISendMethodEvents_FWD_DEFINED__
#define __ISendMethodEvents_FWD_DEFINED__
typedef interface ISendMethodEvents ISendMethodEvents;

#endif 	/* __ISendMethodEvents_FWD_DEFINED__ */


#ifndef __ITransactionResourcePool_FWD_DEFINED__
#define __ITransactionResourcePool_FWD_DEFINED__
typedef interface ITransactionResourcePool ITransactionResourcePool;

#endif 	/* __ITransactionResourcePool_FWD_DEFINED__ */


#ifndef __IMTSCall_FWD_DEFINED__
#define __IMTSCall_FWD_DEFINED__
typedef interface IMTSCall IMTSCall;

#endif 	/* __IMTSCall_FWD_DEFINED__ */


#ifndef __IContextProperties_FWD_DEFINED__
#define __IContextProperties_FWD_DEFINED__
typedef interface IContextProperties IContextProperties;

#endif 	/* __IContextProperties_FWD_DEFINED__ */


#ifndef __IObjPool_FWD_DEFINED__
#define __IObjPool_FWD_DEFINED__
typedef interface IObjPool IObjPool;

#endif 	/* __IObjPool_FWD_DEFINED__ */


#ifndef __ITransactionProperty_FWD_DEFINED__
#define __ITransactionProperty_FWD_DEFINED__
typedef interface ITransactionProperty ITransactionProperty;

#endif 	/* __ITransactionProperty_FWD_DEFINED__ */


#ifndef __IMTSActivity_FWD_DEFINED__
#define __IMTSActivity_FWD_DEFINED__
typedef interface IMTSActivity IMTSActivity;

#endif 	/* __IMTSActivity_FWD_DEFINED__ */


#ifndef __SecurityIdentity_FWD_DEFINED__
#define __SecurityIdentity_FWD_DEFINED__

#ifdef __cplusplus
typedef class SecurityIdentity SecurityIdentity;
#else
typedef struct SecurityIdentity SecurityIdentity;
#endif /* __cplusplus */

#endif 	/* __SecurityIdentity_FWD_DEFINED__ */


#ifndef __SecurityCallers_FWD_DEFINED__
#define __SecurityCallers_FWD_DEFINED__

#ifdef __cplusplus
typedef class SecurityCallers SecurityCallers;
#else
typedef struct SecurityCallers SecurityCallers;
#endif /* __cplusplus */

#endif 	/* __SecurityCallers_FWD_DEFINED__ */


#ifndef __SecurityCallContext_FWD_DEFINED__
#define __SecurityCallContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class SecurityCallContext SecurityCallContext;
#else
typedef struct SecurityCallContext SecurityCallContext;
#endif /* __cplusplus */

#endif 	/* __SecurityCallContext_FWD_DEFINED__ */


#ifndef __GetSecurityCallContextAppObject_FWD_DEFINED__
#define __GetSecurityCallContextAppObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class GetSecurityCallContextAppObject GetSecurityCallContextAppObject;
#else
typedef struct GetSecurityCallContextAppObject GetSecurityCallContextAppObject;
#endif /* __cplusplus */

#endif 	/* __GetSecurityCallContextAppObject_FWD_DEFINED__ */


#ifndef __IContextState_FWD_DEFINED__
#define __IContextState_FWD_DEFINED__
typedef interface IContextState IContextState;

#endif 	/* __IContextState_FWD_DEFINED__ */


#ifndef __Dummy30040732_FWD_DEFINED__
#define __Dummy30040732_FWD_DEFINED__

#ifdef __cplusplus
typedef class Dummy30040732 Dummy30040732;
#else
typedef struct Dummy30040732 Dummy30040732;
#endif /* __cplusplus */

#endif 	/* __Dummy30040732_FWD_DEFINED__ */


#ifndef __ContextInfo_FWD_DEFINED__
#define __ContextInfo_FWD_DEFINED__
typedef interface ContextInfo ContextInfo;

#endif 	/* __ContextInfo_FWD_DEFINED__ */


#ifndef __ContextInfo2_FWD_DEFINED__
#define __ContextInfo2_FWD_DEFINED__
typedef interface ContextInfo2 ContextInfo2;

#endif 	/* __ContextInfo2_FWD_DEFINED__ */


#ifndef __ObjectControl_FWD_DEFINED__
#define __ObjectControl_FWD_DEFINED__
typedef interface ObjectControl ObjectControl;

#endif 	/* __ObjectControl_FWD_DEFINED__ */


#ifndef __TransactionContext_FWD_DEFINED__
#define __TransactionContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class TransactionContext TransactionContext;
#else
typedef struct TransactionContext TransactionContext;
#endif /* __cplusplus */

#endif 	/* __TransactionContext_FWD_DEFINED__ */


#ifndef __TransactionContextEx_FWD_DEFINED__
#define __TransactionContextEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class TransactionContextEx TransactionContextEx;
#else
typedef struct TransactionContextEx TransactionContextEx;
#endif /* __cplusplus */

#endif 	/* __TransactionContextEx_FWD_DEFINED__ */


#ifndef __ByotServerEx_FWD_DEFINED__
#define __ByotServerEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class ByotServerEx ByotServerEx;
#else
typedef struct ByotServerEx ByotServerEx;
#endif /* __cplusplus */

#endif 	/* __ByotServerEx_FWD_DEFINED__ */


#ifndef __CServiceConfig_FWD_DEFINED__
#define __CServiceConfig_FWD_DEFINED__

#ifdef __cplusplus
typedef class CServiceConfig CServiceConfig;
#else
typedef struct CServiceConfig CServiceConfig;
#endif /* __cplusplus */

#endif 	/* __CServiceConfig_FWD_DEFINED__ */


#ifndef __ServicePool_FWD_DEFINED__
#define __ServicePool_FWD_DEFINED__

#ifdef __cplusplus
typedef class ServicePool ServicePool;
#else
typedef struct ServicePool ServicePool;
#endif /* __cplusplus */

#endif 	/* __ServicePool_FWD_DEFINED__ */


#ifndef __ServicePoolConfig_FWD_DEFINED__
#define __ServicePoolConfig_FWD_DEFINED__

#ifdef __cplusplus
typedef class ServicePoolConfig ServicePoolConfig;
#else
typedef struct ServicePoolConfig ServicePoolConfig;
#endif /* __cplusplus */

#endif 	/* __ServicePoolConfig_FWD_DEFINED__ */


#ifndef __SharedProperty_FWD_DEFINED__
#define __SharedProperty_FWD_DEFINED__

#ifdef __cplusplus
typedef class SharedProperty SharedProperty;
#else
typedef struct SharedProperty SharedProperty;
#endif /* __cplusplus */

#endif 	/* __SharedProperty_FWD_DEFINED__ */


#ifndef __SharedPropertyGroup_FWD_DEFINED__
#define __SharedPropertyGroup_FWD_DEFINED__

#ifdef __cplusplus
typedef class SharedPropertyGroup SharedPropertyGroup;
#else
typedef struct SharedPropertyGroup SharedPropertyGroup;
#endif /* __cplusplus */

#endif 	/* __SharedPropertyGroup_FWD_DEFINED__ */


#ifndef __SharedPropertyGroupManager_FWD_DEFINED__
#define __SharedPropertyGroupManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class SharedPropertyGroupManager SharedPropertyGroupManager;
#else
typedef struct SharedPropertyGroupManager SharedPropertyGroupManager;
#endif /* __cplusplus */

#endif 	/* __SharedPropertyGroupManager_FWD_DEFINED__ */


#ifndef __COMEvents_FWD_DEFINED__
#define __COMEvents_FWD_DEFINED__

#ifdef __cplusplus
typedef class COMEvents COMEvents;
#else
typedef struct COMEvents COMEvents;
#endif /* __cplusplus */

#endif 	/* __COMEvents_FWD_DEFINED__ */


#ifndef __CoMTSLocator_FWD_DEFINED__
#define __CoMTSLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class CoMTSLocator CoMTSLocator;
#else
typedef struct CoMTSLocator CoMTSLocator;
#endif /* __cplusplus */

#endif 	/* __CoMTSLocator_FWD_DEFINED__ */


#ifndef __MtsGrp_FWD_DEFINED__
#define __MtsGrp_FWD_DEFINED__

#ifdef __cplusplus
typedef class MtsGrp MtsGrp;
#else
typedef struct MtsGrp MtsGrp;
#endif /* __cplusplus */

#endif 	/* __MtsGrp_FWD_DEFINED__ */


#ifndef __ComServiceEvents_FWD_DEFINED__
#define __ComServiceEvents_FWD_DEFINED__

#ifdef __cplusplus
typedef class ComServiceEvents ComServiceEvents;
#else
typedef struct ComServiceEvents ComServiceEvents;
#endif /* __cplusplus */

#endif 	/* __ComServiceEvents_FWD_DEFINED__ */


#ifndef __ComSystemAppEventData_FWD_DEFINED__
#define __ComSystemAppEventData_FWD_DEFINED__

#ifdef __cplusplus
typedef class ComSystemAppEventData ComSystemAppEventData;
#else
typedef struct ComSystemAppEventData ComSystemAppEventData;
#endif /* __cplusplus */

#endif 	/* __ComSystemAppEventData_FWD_DEFINED__ */


#ifndef __CRMClerk_FWD_DEFINED__
#define __CRMClerk_FWD_DEFINED__

#ifdef __cplusplus
typedef class CRMClerk CRMClerk;
#else
typedef struct CRMClerk CRMClerk;
#endif /* __cplusplus */

#endif 	/* __CRMClerk_FWD_DEFINED__ */


#ifndef __CRMRecoveryClerk_FWD_DEFINED__
#define __CRMRecoveryClerk_FWD_DEFINED__

#ifdef __cplusplus
typedef class CRMRecoveryClerk CRMRecoveryClerk;
#else
typedef struct CRMRecoveryClerk CRMRecoveryClerk;
#endif /* __cplusplus */

#endif 	/* __CRMRecoveryClerk_FWD_DEFINED__ */


#ifndef __LBEvents_FWD_DEFINED__
#define __LBEvents_FWD_DEFINED__

#ifdef __cplusplus
typedef class LBEvents LBEvents;
#else
typedef struct LBEvents LBEvents;
#endif /* __cplusplus */

#endif 	/* __LBEvents_FWD_DEFINED__ */


#ifndef __MessageMover_FWD_DEFINED__
#define __MessageMover_FWD_DEFINED__

#ifdef __cplusplus
typedef class MessageMover MessageMover;
#else
typedef struct MessageMover MessageMover;
#endif /* __cplusplus */

#endif 	/* __MessageMover_FWD_DEFINED__ */


#ifndef __DispenserManager_FWD_DEFINED__
#define __DispenserManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class DispenserManager DispenserManager;
#else
typedef struct DispenserManager DispenserManager;
#endif /* __cplusplus */

#endif 	/* __DispenserManager_FWD_DEFINED__ */


#ifndef __PoolMgr_FWD_DEFINED__
#define __PoolMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class PoolMgr PoolMgr;
#else
typedef struct PoolMgr PoolMgr;
#endif /* __cplusplus */

#endif 	/* __PoolMgr_FWD_DEFINED__ */


#ifndef __EventServer_FWD_DEFINED__
#define __EventServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class EventServer EventServer;
#else
typedef struct EventServer EventServer;
#endif /* __cplusplus */

#endif 	/* __EventServer_FWD_DEFINED__ */


#ifndef __TrackerServer_FWD_DEFINED__
#define __TrackerServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class TrackerServer TrackerServer;
#else
typedef struct TrackerServer TrackerServer;
#endif /* __cplusplus */

#endif 	/* __TrackerServer_FWD_DEFINED__ */


#ifndef __AppDomainHelper_FWD_DEFINED__
#define __AppDomainHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class AppDomainHelper AppDomainHelper;
#else
typedef struct AppDomainHelper AppDomainHelper;
#endif /* __cplusplus */

#endif 	/* __AppDomainHelper_FWD_DEFINED__ */


#ifndef __ClrAssemblyLocator_FWD_DEFINED__
#define __ClrAssemblyLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClrAssemblyLocator ClrAssemblyLocator;
#else
typedef struct ClrAssemblyLocator ClrAssemblyLocator;
#endif /* __cplusplus */

#endif 	/* __ClrAssemblyLocator_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "ocidl.h"
#include "comadmin.h"
#include "transact.h"
#include "txdtc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_autosvcs_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// -----------------------------------------------------------------------
// svcintfs.h -- Microsoft COM+ Services 1.0 Programming Interfaces       
//                                                                        
// This file provides the prototypes for the APIs and COM interfaces      
// for applications using COM+ Services.                                  
//                                                                        
// COM+ Services 1.0                                                      
//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.
// -----------------------------------------------------------------------
#include <objbase.h>
#ifndef DECLSPEC_UUID
#if (_MSC_VER >= 1100) && defined (__cplusplus)
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif
#endif



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0000_v0_0_s_ifspec;

#ifndef __ISecurityIdentityColl_INTERFACE_DEFINED__
#define __ISecurityIdentityColl_INTERFACE_DEFINED__

/* interface ISecurityIdentityColl */
/* [unique][helpcontext][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ISecurityIdentityColl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CAFC823C-B441-11d1-B82B-0000F8757E2A")
    ISecurityIdentityColl : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][helpcontext][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem) = 0;
        
        virtual /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISecurityIdentityCollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISecurityIdentityColl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISecurityIdentityColl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISecurityIdentityColl * This,
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
        
        DECLSPEC_XFGVIRT(ISecurityIdentityColl, get_Count)
        /* [helpstring][propget][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISecurityIdentityColl, get_Item)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem);
        
        DECLSPEC_XFGVIRT(ISecurityIdentityColl, get__NewEnum)
        /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISecurityIdentityColl * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        END_INTERFACE
    } ISecurityIdentityCollVtbl;

    interface ISecurityIdentityColl
    {
        CONST_VTBL struct ISecurityIdentityCollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISecurityIdentityColl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISecurityIdentityColl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISecurityIdentityColl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISecurityIdentityColl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISecurityIdentityColl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISecurityIdentityColl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISecurityIdentityColl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISecurityIdentityColl_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISecurityIdentityColl_get_Item(This,name,pItem)	\
    ( (This)->lpVtbl -> get_Item(This,name,pItem) ) 

#define ISecurityIdentityColl_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISecurityIdentityColl_INTERFACE_DEFINED__ */


#ifndef __ISecurityCallersColl_INTERFACE_DEFINED__
#define __ISecurityCallersColl_INTERFACE_DEFINED__

/* interface ISecurityCallersColl */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ISecurityCallersColl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CAFC823D-B441-11d1-B82B-0000F8757E2A")
    ISecurityCallersColl : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][helpcontext][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISecurityIdentityColl **pObj) = 0;
        
        virtual /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISecurityCallersCollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISecurityCallersColl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISecurityCallersColl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISecurityCallersColl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISecurityCallersColl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISecurityCallersColl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISecurityCallersColl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISecurityCallersColl * This,
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
        
        DECLSPEC_XFGVIRT(ISecurityCallersColl, get_Count)
        /* [helpstring][propget][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISecurityCallersColl * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISecurityCallersColl, get_Item)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISecurityCallersColl * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISecurityIdentityColl **pObj);
        
        DECLSPEC_XFGVIRT(ISecurityCallersColl, get__NewEnum)
        /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISecurityCallersColl * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        END_INTERFACE
    } ISecurityCallersCollVtbl;

    interface ISecurityCallersColl
    {
        CONST_VTBL struct ISecurityCallersCollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISecurityCallersColl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISecurityCallersColl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISecurityCallersColl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISecurityCallersColl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISecurityCallersColl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISecurityCallersColl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISecurityCallersColl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISecurityCallersColl_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISecurityCallersColl_get_Item(This,lIndex,pObj)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pObj) ) 

#define ISecurityCallersColl_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISecurityCallersColl_INTERFACE_DEFINED__ */


#ifndef __ISecurityCallContext_INTERFACE_DEFINED__
#define __ISecurityCallContext_INTERFACE_DEFINED__

/* interface ISecurityCallContext */
/* [unique][helpcontext][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ISecurityCallContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CAFC823E-B441-11d1-B82B-0000F8757E2A")
    ISecurityCallContext : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][helpcontext][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem) = 0;
        
        virtual /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsCallerInRole( 
            __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfInRole) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsSecurityEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsEnabled) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsUserInRole( 
            /* [in] */ __RPC__in VARIANT *pUser,
            /* [in] */ __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfInRole) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISecurityCallContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISecurityCallContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISecurityCallContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISecurityCallContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISecurityCallContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISecurityCallContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISecurityCallContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISecurityCallContext * This,
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
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, get_Count)
        /* [helpstring][propget][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISecurityCallContext * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, get_Item)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISecurityCallContext * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem);
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, get__NewEnum)
        /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISecurityCallContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, IsCallerInRole)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsCallerInRole )( 
            __RPC__in ISecurityCallContext * This,
            __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfInRole);
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, IsSecurityEnabled)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsSecurityEnabled )( 
            __RPC__in ISecurityCallContext * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsEnabled);
        
        DECLSPEC_XFGVIRT(ISecurityCallContext, IsUserInRole)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsUserInRole )( 
            __RPC__in ISecurityCallContext * This,
            /* [in] */ __RPC__in VARIANT *pUser,
            /* [in] */ __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfInRole);
        
        END_INTERFACE
    } ISecurityCallContextVtbl;

    interface ISecurityCallContext
    {
        CONST_VTBL struct ISecurityCallContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISecurityCallContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISecurityCallContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISecurityCallContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISecurityCallContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISecurityCallContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISecurityCallContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISecurityCallContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISecurityCallContext_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISecurityCallContext_get_Item(This,name,pItem)	\
    ( (This)->lpVtbl -> get_Item(This,name,pItem) ) 

#define ISecurityCallContext_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define ISecurityCallContext_IsCallerInRole(This,bstrRole,pfInRole)	\
    ( (This)->lpVtbl -> IsCallerInRole(This,bstrRole,pfInRole) ) 

#define ISecurityCallContext_IsSecurityEnabled(This,pfIsEnabled)	\
    ( (This)->lpVtbl -> IsSecurityEnabled(This,pfIsEnabled) ) 

#define ISecurityCallContext_IsUserInRole(This,pUser,bstrRole,pfInRole)	\
    ( (This)->lpVtbl -> IsUserInRole(This,pUser,bstrRole,pfInRole) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISecurityCallContext_INTERFACE_DEFINED__ */


#ifndef __IGetSecurityCallContext_INTERFACE_DEFINED__
#define __IGetSecurityCallContext_INTERFACE_DEFINED__

/* interface IGetSecurityCallContext */
/* [unique][helpcontext][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IGetSecurityCallContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CAFC823F-B441-11d1-B82B-0000F8757E2A")
    IGetSecurityCallContext : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetSecurityCallContext( 
            /* [retval][out] */ __RPC__deref_out_opt ISecurityCallContext **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetSecurityCallContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetSecurityCallContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetSecurityCallContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetSecurityCallContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGetSecurityCallContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGetSecurityCallContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGetSecurityCallContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGetSecurityCallContext * This,
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
        
        DECLSPEC_XFGVIRT(IGetSecurityCallContext, GetSecurityCallContext)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityCallContext )( 
            __RPC__in IGetSecurityCallContext * This,
            /* [retval][out] */ __RPC__deref_out_opt ISecurityCallContext **ppObject);
        
        END_INTERFACE
    } IGetSecurityCallContextVtbl;

    interface IGetSecurityCallContext
    {
        CONST_VTBL struct IGetSecurityCallContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetSecurityCallContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetSecurityCallContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetSecurityCallContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetSecurityCallContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGetSecurityCallContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGetSecurityCallContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGetSecurityCallContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGetSecurityCallContext_GetSecurityCallContext(This,ppObject)	\
    ( (This)->lpVtbl -> GetSecurityCallContext(This,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetSecurityCallContext_INTERFACE_DEFINED__ */


#ifndef __SecurityProperty_INTERFACE_DEFINED__
#define __SecurityProperty_INTERFACE_DEFINED__

/* interface SecurityProperty */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_SecurityProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E74A7215-014D-11d1-A63C-00A0C911B4E0")
    SecurityProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetDirectCallerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetDirectCreatorName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetOriginalCallerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetOriginalCreatorName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct SecurityPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in SecurityProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in SecurityProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in SecurityProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in SecurityProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in SecurityProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in SecurityProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            SecurityProperty * This,
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
        
        DECLSPEC_XFGVIRT(SecurityProperty, GetDirectCallerName)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetDirectCallerName )( 
            __RPC__in SecurityProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName);
        
        DECLSPEC_XFGVIRT(SecurityProperty, GetDirectCreatorName)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetDirectCreatorName )( 
            __RPC__in SecurityProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName);
        
        DECLSPEC_XFGVIRT(SecurityProperty, GetOriginalCallerName)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetOriginalCallerName )( 
            __RPC__in SecurityProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName);
        
        DECLSPEC_XFGVIRT(SecurityProperty, GetOriginalCreatorName)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetOriginalCreatorName )( 
            __RPC__in SecurityProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName);
        
        END_INTERFACE
    } SecurityPropertyVtbl;

    interface SecurityProperty
    {
        CONST_VTBL struct SecurityPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define SecurityProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define SecurityProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define SecurityProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define SecurityProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define SecurityProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define SecurityProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define SecurityProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define SecurityProperty_GetDirectCallerName(This,bstrUserName)	\
    ( (This)->lpVtbl -> GetDirectCallerName(This,bstrUserName) ) 

#define SecurityProperty_GetDirectCreatorName(This,bstrUserName)	\
    ( (This)->lpVtbl -> GetDirectCreatorName(This,bstrUserName) ) 

#define SecurityProperty_GetOriginalCallerName(This,bstrUserName)	\
    ( (This)->lpVtbl -> GetOriginalCallerName(This,bstrUserName) ) 

#define SecurityProperty_GetOriginalCreatorName(This,bstrUserName)	\
    ( (This)->lpVtbl -> GetOriginalCreatorName(This,bstrUserName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __SecurityProperty_INTERFACE_DEFINED__ */


#ifndef __ContextInfo_INTERFACE_DEFINED__
#define __ContextInfo_INTERFACE_DEFINED__

/* interface ContextInfo */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ContextInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19A5A02C-0AC8-11d2-B286-00C04F8EF934")
    ContextInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsInTransaction( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInTx) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTx) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTransactionId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTxId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetActivityId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrActivityId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetContextId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCtxId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ContextInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ContextInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ContextInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ContextInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ContextInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ContextInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ContextInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ContextInfo * This,
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
        
        DECLSPEC_XFGVIRT(ContextInfo, IsInTransaction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsInTransaction )( 
            __RPC__in ContextInfo * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInTx);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetTransaction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTransaction )( 
            __RPC__in ContextInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTx);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetTransactionId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTransactionId )( 
            __RPC__in ContextInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTxId);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetActivityId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetActivityId )( 
            __RPC__in ContextInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrActivityId);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetContextId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetContextId )( 
            __RPC__in ContextInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCtxId);
        
        END_INTERFACE
    } ContextInfoVtbl;

    interface ContextInfo
    {
        CONST_VTBL struct ContextInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ContextInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ContextInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ContextInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ContextInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ContextInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ContextInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ContextInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ContextInfo_IsInTransaction(This,pbIsInTx)	\
    ( (This)->lpVtbl -> IsInTransaction(This,pbIsInTx) ) 

#define ContextInfo_GetTransaction(This,ppTx)	\
    ( (This)->lpVtbl -> GetTransaction(This,ppTx) ) 

#define ContextInfo_GetTransactionId(This,pbstrTxId)	\
    ( (This)->lpVtbl -> GetTransactionId(This,pbstrTxId) ) 

#define ContextInfo_GetActivityId(This,pbstrActivityId)	\
    ( (This)->lpVtbl -> GetActivityId(This,pbstrActivityId) ) 

#define ContextInfo_GetContextId(This,pbstrCtxId)	\
    ( (This)->lpVtbl -> GetContextId(This,pbstrCtxId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ContextInfo_INTERFACE_DEFINED__ */


#ifndef __ContextInfo2_INTERFACE_DEFINED__
#define __ContextInfo2_INTERFACE_DEFINED__

/* interface ContextInfo2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ContextInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c99d6e75-2375-11d4-8331-00c04f605588")
    ContextInfo2 : public ContextInfo
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPartitionId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20000) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetApplicationId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20001) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetApplicationInstanceId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20002) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ContextInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ContextInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ContextInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ContextInfo2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ContextInfo2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ContextInfo2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ContextInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ContextInfo2 * This,
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
        
        DECLSPEC_XFGVIRT(ContextInfo, IsInTransaction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsInTransaction )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInTx);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetTransaction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTransaction )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTx);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetTransactionId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTransactionId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTxId);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetActivityId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetActivityId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrActivityId);
        
        DECLSPEC_XFGVIRT(ContextInfo, GetContextId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetContextId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCtxId);
        
        DECLSPEC_XFGVIRT(ContextInfo2, GetPartitionId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPartitionId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20000);
        
        DECLSPEC_XFGVIRT(ContextInfo2, GetApplicationId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20001);
        
        DECLSPEC_XFGVIRT(ContextInfo2, GetApplicationInstanceId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationInstanceId )( 
            __RPC__in ContextInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *__MIDL__ContextInfo20002);
        
        END_INTERFACE
    } ContextInfo2Vtbl;

    interface ContextInfo2
    {
        CONST_VTBL struct ContextInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ContextInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ContextInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ContextInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ContextInfo2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ContextInfo2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ContextInfo2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ContextInfo2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ContextInfo2_IsInTransaction(This,pbIsInTx)	\
    ( (This)->lpVtbl -> IsInTransaction(This,pbIsInTx) ) 

#define ContextInfo2_GetTransaction(This,ppTx)	\
    ( (This)->lpVtbl -> GetTransaction(This,ppTx) ) 

#define ContextInfo2_GetTransactionId(This,pbstrTxId)	\
    ( (This)->lpVtbl -> GetTransactionId(This,pbstrTxId) ) 

#define ContextInfo2_GetActivityId(This,pbstrActivityId)	\
    ( (This)->lpVtbl -> GetActivityId(This,pbstrActivityId) ) 

#define ContextInfo2_GetContextId(This,pbstrCtxId)	\
    ( (This)->lpVtbl -> GetContextId(This,pbstrCtxId) ) 


#define ContextInfo2_GetPartitionId(This,__MIDL__ContextInfo20000)	\
    ( (This)->lpVtbl -> GetPartitionId(This,__MIDL__ContextInfo20000) ) 

#define ContextInfo2_GetApplicationId(This,__MIDL__ContextInfo20001)	\
    ( (This)->lpVtbl -> GetApplicationId(This,__MIDL__ContextInfo20001) ) 

#define ContextInfo2_GetApplicationInstanceId(This,__MIDL__ContextInfo20002)	\
    ( (This)->lpVtbl -> GetApplicationInstanceId(This,__MIDL__ContextInfo20002) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ContextInfo2_INTERFACE_DEFINED__ */


#ifndef __ObjectContext_INTERFACE_DEFINED__
#define __ObjectContext_INTERFACE_DEFINED__

/* interface ObjectContext */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ObjectContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74C08646-CEDB-11CF-8B49-00AA00B8A790")
    ObjectContext : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ __RPC__in BSTR bstrProgID,
            /* [retval][out] */ __RPC__out VARIANT *pObject) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetComplete( void) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetAbort( void) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE EnableCommit( void) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE DisableCommit( void) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsInTransaction( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInTx) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsSecurityEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsEnabled) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE IsCallerInRole( 
            __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbInRole) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem) = 0;
        
        virtual /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum) = 0;
        
        virtual /* [helpstring][helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__deref_out_opt SecurityProperty **ppSecurityProperty) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ContextInfo( 
            /* [retval][out] */ __RPC__deref_out_opt ContextInfo **ppContextInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ObjectContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ObjectContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ObjectContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ObjectContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ObjectContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ObjectContext * This,
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
        
        DECLSPEC_XFGVIRT(ObjectContext, CreateInstance)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in ObjectContext * This,
            /* [in] */ __RPC__in BSTR bstrProgID,
            /* [retval][out] */ __RPC__out VARIANT *pObject);
        
        DECLSPEC_XFGVIRT(ObjectContext, SetComplete)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetComplete )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(ObjectContext, SetAbort)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetAbort )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(ObjectContext, EnableCommit)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *EnableCommit )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(ObjectContext, DisableCommit)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *DisableCommit )( 
            __RPC__in ObjectContext * This);
        
        DECLSPEC_XFGVIRT(ObjectContext, IsInTransaction)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsInTransaction )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInTx);
        
        DECLSPEC_XFGVIRT(ObjectContext, IsSecurityEnabled)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsSecurityEnabled )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsEnabled);
        
        DECLSPEC_XFGVIRT(ObjectContext, IsCallerInRole)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *IsCallerInRole )( 
            __RPC__in ObjectContext * This,
            __RPC__in BSTR bstrRole,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbInRole);
        
        DECLSPEC_XFGVIRT(ObjectContext, get_Count)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ObjectContext, get_Item)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ObjectContext * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pItem);
        
        DECLSPEC_XFGVIRT(ObjectContext, get__NewEnum)
        /* [helpstring][helpcontext][restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnum);
        
        DECLSPEC_XFGVIRT(ObjectContext, get_Security)
        /* [helpstring][helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__deref_out_opt SecurityProperty **ppSecurityProperty);
        
        DECLSPEC_XFGVIRT(ObjectContext, get_ContextInfo)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ContextInfo )( 
            __RPC__in ObjectContext * This,
            /* [retval][out] */ __RPC__deref_out_opt ContextInfo **ppContextInfo);
        
        END_INTERFACE
    } ObjectContextVtbl;

    interface ObjectContext
    {
        CONST_VTBL struct ObjectContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ObjectContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ObjectContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ObjectContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ObjectContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ObjectContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ObjectContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ObjectContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ObjectContext_CreateInstance(This,bstrProgID,pObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,bstrProgID,pObject) ) 

#define ObjectContext_SetComplete(This)	\
    ( (This)->lpVtbl -> SetComplete(This) ) 

#define ObjectContext_SetAbort(This)	\
    ( (This)->lpVtbl -> SetAbort(This) ) 

#define ObjectContext_EnableCommit(This)	\
    ( (This)->lpVtbl -> EnableCommit(This) ) 

#define ObjectContext_DisableCommit(This)	\
    ( (This)->lpVtbl -> DisableCommit(This) ) 

#define ObjectContext_IsInTransaction(This,pbIsInTx)	\
    ( (This)->lpVtbl -> IsInTransaction(This,pbIsInTx) ) 

#define ObjectContext_IsSecurityEnabled(This,pbIsEnabled)	\
    ( (This)->lpVtbl -> IsSecurityEnabled(This,pbIsEnabled) ) 

#define ObjectContext_IsCallerInRole(This,bstrRole,pbInRole)	\
    ( (This)->lpVtbl -> IsCallerInRole(This,bstrRole,pbInRole) ) 

#define ObjectContext_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ObjectContext_get_Item(This,name,pItem)	\
    ( (This)->lpVtbl -> get_Item(This,name,pItem) ) 

#define ObjectContext_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define ObjectContext_get_Security(This,ppSecurityProperty)	\
    ( (This)->lpVtbl -> get_Security(This,ppSecurityProperty) ) 

#define ObjectContext_get_ContextInfo(This,ppContextInfo)	\
    ( (This)->lpVtbl -> get_ContextInfo(This,ppContextInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ObjectContext_INTERFACE_DEFINED__ */


#ifndef __ITransactionContextEx_INTERFACE_DEFINED__
#define __ITransactionContextEx_INTERFACE_DEFINED__

/* interface ITransactionContextEx */
/* [unique][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITransactionContextEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7999FC22-D3C6-11CF-ACAB-00A024A55AEF")
    ITransactionContextEx : public IUnknown
    {
    public:
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject) = 0;
        
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionContextExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionContextEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionContextEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionContextEx * This);
        
        DECLSPEC_XFGVIRT(ITransactionContextEx, CreateInstance)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in ITransactionContextEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject);
        
        DECLSPEC_XFGVIRT(ITransactionContextEx, Commit)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in ITransactionContextEx * This);
        
        DECLSPEC_XFGVIRT(ITransactionContextEx, Abort)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in ITransactionContextEx * This);
        
        END_INTERFACE
    } ITransactionContextExVtbl;

    interface ITransactionContextEx
    {
        CONST_VTBL struct ITransactionContextExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionContextEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionContextEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionContextEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionContextEx_CreateInstance(This,rclsid,riid,pObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,rclsid,riid,pObject) ) 

#define ITransactionContextEx_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#define ITransactionContextEx_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionContextEx_INTERFACE_DEFINED__ */


#ifndef __ITransactionContext_INTERFACE_DEFINED__
#define __ITransactionContext_INTERFACE_DEFINED__

/* interface ITransactionContext */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITransactionContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7999FC21-D3C6-11CF-ACAB-00A024A55AEF")
    ITransactionContext : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ __RPC__in BSTR pszProgId,
            /* [retval][out] */ __RPC__out VARIANT *pObject) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITransactionContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITransactionContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITransactionContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITransactionContext * This,
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
        
        DECLSPEC_XFGVIRT(ITransactionContext, CreateInstance)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in ITransactionContext * This,
            /* [in] */ __RPC__in BSTR pszProgId,
            /* [retval][out] */ __RPC__out VARIANT *pObject);
        
        DECLSPEC_XFGVIRT(ITransactionContext, Commit)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in ITransactionContext * This);
        
        DECLSPEC_XFGVIRT(ITransactionContext, Abort)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in ITransactionContext * This);
        
        END_INTERFACE
    } ITransactionContextVtbl;

    interface ITransactionContext
    {
        CONST_VTBL struct ITransactionContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITransactionContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITransactionContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITransactionContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITransactionContext_CreateInstance(This,pszProgId,pObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,pszProgId,pObject) ) 

#define ITransactionContext_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#define ITransactionContext_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionContext_INTERFACE_DEFINED__ */


#ifndef __ICreateWithTransactionEx_INTERFACE_DEFINED__
#define __ICreateWithTransactionEx_INTERFACE_DEFINED__

/* interface ICreateWithTransactionEx */
/* [unique][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICreateWithTransactionEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("455ACF57-5345-11d2-99CF-00C04F797BC9")
    ICreateWithTransactionEx : public IUnknown
    {
    public:
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateWithTransactionExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICreateWithTransactionEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICreateWithTransactionEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICreateWithTransactionEx * This);
        
        DECLSPEC_XFGVIRT(ICreateWithTransactionEx, CreateInstance)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in ICreateWithTransactionEx * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject);
        
        END_INTERFACE
    } ICreateWithTransactionExVtbl;

    interface ICreateWithTransactionEx
    {
        CONST_VTBL struct ICreateWithTransactionExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateWithTransactionEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateWithTransactionEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateWithTransactionEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateWithTransactionEx_CreateInstance(This,pTransaction,rclsid,riid,pObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,pTransaction,rclsid,riid,pObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateWithTransactionEx_INTERFACE_DEFINED__ */


#ifndef __ICreateWithLocalTransaction_INTERFACE_DEFINED__
#define __ICreateWithLocalTransaction_INTERFACE_DEFINED__

/* interface ICreateWithLocalTransaction */
/* [unique][helpcontext][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_ICreateWithLocalTransaction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("227AC7A8-8423-42ce-B7CF-03061EC9AAA3")
    ICreateWithLocalTransaction : public IUnknown
    {
    public:
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE CreateInstanceWithSysTx( 
            /* [in] */ IUnknown *pTransaction,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **pObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateWithLocalTransactionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateWithLocalTransaction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateWithLocalTransaction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateWithLocalTransaction * This);
        
        DECLSPEC_XFGVIRT(ICreateWithLocalTransaction, CreateInstanceWithSysTx)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CreateInstanceWithSysTx )( 
            ICreateWithLocalTransaction * This,
            /* [in] */ IUnknown *pTransaction,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **pObject);
        
        END_INTERFACE
    } ICreateWithLocalTransactionVtbl;

    interface ICreateWithLocalTransaction
    {
        CONST_VTBL struct ICreateWithLocalTransactionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateWithLocalTransaction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateWithLocalTransaction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateWithLocalTransaction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateWithLocalTransaction_CreateInstanceWithSysTx(This,pTransaction,rclsid,riid,pObject)	\
    ( (This)->lpVtbl -> CreateInstanceWithSysTx(This,pTransaction,rclsid,riid,pObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateWithLocalTransaction_INTERFACE_DEFINED__ */


#ifndef __ICreateWithTipTransactionEx_INTERFACE_DEFINED__
#define __ICreateWithTipTransactionEx_INTERFACE_DEFINED__

/* interface ICreateWithTipTransactionEx */
/* [unique][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICreateWithTipTransactionEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("455ACF59-5345-11d2-99CF-00C04F797BC9")
    ICreateWithTipTransactionEx : public IUnknown
    {
    public:
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ __RPC__in BSTR bstrTipUrl,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateWithTipTransactionExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICreateWithTipTransactionEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICreateWithTipTransactionEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICreateWithTipTransactionEx * This);
        
        DECLSPEC_XFGVIRT(ICreateWithTipTransactionEx, CreateInstance)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in ICreateWithTipTransactionEx * This,
            /* [in] */ __RPC__in BSTR bstrTipUrl,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pObject);
        
        END_INTERFACE
    } ICreateWithTipTransactionExVtbl;

    interface ICreateWithTipTransactionEx
    {
        CONST_VTBL struct ICreateWithTipTransactionExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateWithTipTransactionEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateWithTipTransactionEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateWithTipTransactionEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateWithTipTransactionEx_CreateInstance(This,bstrTipUrl,rclsid,riid,pObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,bstrTipUrl,rclsid,riid,pObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateWithTipTransactionEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0013 */
/* [local] */ 

#pragma deprecated (ICreateWithTipTransactionEx)
typedef unsigned __int64 MTS_OBJID;

typedef unsigned __int64 MTS_RESID;

typedef unsigned __int64 ULONG64;

#ifndef _COMSVCSEVENTINFO_
#define _COMSVCSEVENTINFO_
typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][hidden] */ struct __MIDL___MIDL_itf_autosvcs_0000_0013_0001
    {
    DWORD cbSize;
    DWORD dwPid;
    LONGLONG lTime;
    LONG lMicroTime;
    LONGLONG perfCount;
    GUID guidApp;
    LPOLESTR sMachineName;
    } 	COMSVCSEVENTINFO;

#endif // _COMSVCSEVENTINFO_


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0013_v0_0_s_ifspec;

#ifndef __IComLTxEvents_INTERFACE_DEFINED__
#define __IComLTxEvents_INTERFACE_DEFINED__

/* interface IComLTxEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComLTxEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("605CF82C-578E-4298-975D-82BABCD9E053")
    IComLTxEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnLtxTransactionStart( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            GUID tsid,
            BOOL fRoot,
            int nIsolationLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLtxTransactionPrepare( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            BOOL fVote) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLtxTransactionAbort( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLtxTransactionCommit( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLtxTransactionPromote( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            GUID txnId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComLTxEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComLTxEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComLTxEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComLTxEvents * This);
        
        DECLSPEC_XFGVIRT(IComLTxEvents, OnLtxTransactionStart)
        HRESULT ( STDMETHODCALLTYPE *OnLtxTransactionStart )( 
            __RPC__in IComLTxEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            GUID tsid,
            BOOL fRoot,
            int nIsolationLevel);
        
        DECLSPEC_XFGVIRT(IComLTxEvents, OnLtxTransactionPrepare)
        HRESULT ( STDMETHODCALLTYPE *OnLtxTransactionPrepare )( 
            __RPC__in IComLTxEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            BOOL fVote);
        
        DECLSPEC_XFGVIRT(IComLTxEvents, OnLtxTransactionAbort)
        HRESULT ( STDMETHODCALLTYPE *OnLtxTransactionAbort )( 
            __RPC__in IComLTxEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx);
        
        DECLSPEC_XFGVIRT(IComLTxEvents, OnLtxTransactionCommit)
        HRESULT ( STDMETHODCALLTYPE *OnLtxTransactionCommit )( 
            __RPC__in IComLTxEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx);
        
        DECLSPEC_XFGVIRT(IComLTxEvents, OnLtxTransactionPromote)
        HRESULT ( STDMETHODCALLTYPE *OnLtxTransactionPromote )( 
            __RPC__in IComLTxEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            GUID guidLtx,
            GUID txnId);
        
        END_INTERFACE
    } IComLTxEventsVtbl;

    interface IComLTxEvents
    {
        CONST_VTBL struct IComLTxEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComLTxEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComLTxEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComLTxEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComLTxEvents_OnLtxTransactionStart(This,pInfo,guidLtx,tsid,fRoot,nIsolationLevel)	\
    ( (This)->lpVtbl -> OnLtxTransactionStart(This,pInfo,guidLtx,tsid,fRoot,nIsolationLevel) ) 

#define IComLTxEvents_OnLtxTransactionPrepare(This,pInfo,guidLtx,fVote)	\
    ( (This)->lpVtbl -> OnLtxTransactionPrepare(This,pInfo,guidLtx,fVote) ) 

#define IComLTxEvents_OnLtxTransactionAbort(This,pInfo,guidLtx)	\
    ( (This)->lpVtbl -> OnLtxTransactionAbort(This,pInfo,guidLtx) ) 

#define IComLTxEvents_OnLtxTransactionCommit(This,pInfo,guidLtx)	\
    ( (This)->lpVtbl -> OnLtxTransactionCommit(This,pInfo,guidLtx) ) 

#define IComLTxEvents_OnLtxTransactionPromote(This,pInfo,guidLtx,txnId)	\
    ( (This)->lpVtbl -> OnLtxTransactionPromote(This,pInfo,guidLtx,txnId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComLTxEvents_INTERFACE_DEFINED__ */


#ifndef __IComUserEvent_INTERFACE_DEFINED__
#define __IComUserEvent_INTERFACE_DEFINED__

/* interface IComUserEvent */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComUserEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A4-2E50-11d2-98A5-00C04F8EE1C4")
    IComUserEvent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnUserEvent( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in VARIANT *pvarEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComUserEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComUserEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComUserEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComUserEvent * This);
        
        DECLSPEC_XFGVIRT(IComUserEvent, OnUserEvent)
        HRESULT ( STDMETHODCALLTYPE *OnUserEvent )( 
            __RPC__in IComUserEvent * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in VARIANT *pvarEvent);
        
        END_INTERFACE
    } IComUserEventVtbl;

    interface IComUserEvent
    {
        CONST_VTBL struct IComUserEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComUserEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComUserEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComUserEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComUserEvent_OnUserEvent(This,pInfo,pvarEvent)	\
    ( (This)->lpVtbl -> OnUserEvent(This,pInfo,pvarEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComUserEvent_INTERFACE_DEFINED__ */


#ifndef __IComThreadEvents_INTERFACE_DEFINED__
#define __IComThreadEvents_INTERFACE_DEFINED__

/* interface IComThreadEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComThreadEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A5-2E50-11d2-98A5-00C04F8EE1C4")
    IComThreadEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnThreadStart( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTheadCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadTerminate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTheadCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadBindToApartment( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 AptID,
            /* [in] */ DWORD dwActCnt,
            /* [in] */ DWORD dwLowCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadUnBind( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 AptID,
            /* [in] */ DWORD dwActCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadWorkEnque( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadWorkPrivate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadWorkPublic( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadWorkRedirect( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen,
            /* [in] */ ULONG64 ThreadNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadWorkReject( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadAssignApartment( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ ULONG64 AptID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadUnassignApartment( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 AptID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComThreadEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComThreadEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComThreadEvents * This);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadStart)
        HRESULT ( STDMETHODCALLTYPE *OnThreadStart )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTheadCnt);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadTerminate)
        HRESULT ( STDMETHODCALLTYPE *OnThreadTerminate )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTheadCnt);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadBindToApartment)
        HRESULT ( STDMETHODCALLTYPE *OnThreadBindToApartment )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 AptID,
            /* [in] */ DWORD dwActCnt,
            /* [in] */ DWORD dwLowCnt);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadUnBind)
        HRESULT ( STDMETHODCALLTYPE *OnThreadUnBind )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 AptID,
            /* [in] */ DWORD dwActCnt);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadWorkEnque)
        HRESULT ( STDMETHODCALLTYPE *OnThreadWorkEnque )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadWorkPrivate)
        HRESULT ( STDMETHODCALLTYPE *OnThreadWorkPrivate )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadWorkPublic)
        HRESULT ( STDMETHODCALLTYPE *OnThreadWorkPublic )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadWorkRedirect)
        HRESULT ( STDMETHODCALLTYPE *OnThreadWorkRedirect )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen,
            /* [in] */ ULONG64 ThreadNum);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadWorkReject)
        HRESULT ( STDMETHODCALLTYPE *OnThreadWorkReject )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ThreadID,
            /* [in] */ ULONG64 MsgWorkID,
            /* [in] */ DWORD QueueLen);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadAssignApartment)
        HRESULT ( STDMETHODCALLTYPE *OnThreadAssignApartment )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ ULONG64 AptID);
        
        DECLSPEC_XFGVIRT(IComThreadEvents, OnThreadUnassignApartment)
        HRESULT ( STDMETHODCALLTYPE *OnThreadUnassignApartment )( 
            __RPC__in IComThreadEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 AptID);
        
        END_INTERFACE
    } IComThreadEventsVtbl;

    interface IComThreadEvents
    {
        CONST_VTBL struct IComThreadEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComThreadEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComThreadEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComThreadEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComThreadEvents_OnThreadStart(This,pInfo,ThreadID,dwThread,dwTheadCnt)	\
    ( (This)->lpVtbl -> OnThreadStart(This,pInfo,ThreadID,dwThread,dwTheadCnt) ) 

#define IComThreadEvents_OnThreadTerminate(This,pInfo,ThreadID,dwThread,dwTheadCnt)	\
    ( (This)->lpVtbl -> OnThreadTerminate(This,pInfo,ThreadID,dwThread,dwTheadCnt) ) 

#define IComThreadEvents_OnThreadBindToApartment(This,pInfo,ThreadID,AptID,dwActCnt,dwLowCnt)	\
    ( (This)->lpVtbl -> OnThreadBindToApartment(This,pInfo,ThreadID,AptID,dwActCnt,dwLowCnt) ) 

#define IComThreadEvents_OnThreadUnBind(This,pInfo,ThreadID,AptID,dwActCnt)	\
    ( (This)->lpVtbl -> OnThreadUnBind(This,pInfo,ThreadID,AptID,dwActCnt) ) 

#define IComThreadEvents_OnThreadWorkEnque(This,pInfo,ThreadID,MsgWorkID,QueueLen)	\
    ( (This)->lpVtbl -> OnThreadWorkEnque(This,pInfo,ThreadID,MsgWorkID,QueueLen) ) 

#define IComThreadEvents_OnThreadWorkPrivate(This,pInfo,ThreadID,MsgWorkID)	\
    ( (This)->lpVtbl -> OnThreadWorkPrivate(This,pInfo,ThreadID,MsgWorkID) ) 

#define IComThreadEvents_OnThreadWorkPublic(This,pInfo,ThreadID,MsgWorkID,QueueLen)	\
    ( (This)->lpVtbl -> OnThreadWorkPublic(This,pInfo,ThreadID,MsgWorkID,QueueLen) ) 

#define IComThreadEvents_OnThreadWorkRedirect(This,pInfo,ThreadID,MsgWorkID,QueueLen,ThreadNum)	\
    ( (This)->lpVtbl -> OnThreadWorkRedirect(This,pInfo,ThreadID,MsgWorkID,QueueLen,ThreadNum) ) 

#define IComThreadEvents_OnThreadWorkReject(This,pInfo,ThreadID,MsgWorkID,QueueLen)	\
    ( (This)->lpVtbl -> OnThreadWorkReject(This,pInfo,ThreadID,MsgWorkID,QueueLen) ) 

#define IComThreadEvents_OnThreadAssignApartment(This,pInfo,guidActivity,AptID)	\
    ( (This)->lpVtbl -> OnThreadAssignApartment(This,pInfo,guidActivity,AptID) ) 

#define IComThreadEvents_OnThreadUnassignApartment(This,pInfo,AptID)	\
    ( (This)->lpVtbl -> OnThreadUnassignApartment(This,pInfo,AptID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComThreadEvents_INTERFACE_DEFINED__ */


#ifndef __IComAppEvents_INTERFACE_DEFINED__
#define __IComAppEvents_INTERFACE_DEFINED__

/* interface IComAppEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComAppEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A6-2E50-11d2-98A5-00C04F8EE1C4")
    IComAppEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnAppActivation( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppShutdown( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppForceShutdown( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComAppEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComAppEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComAppEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComAppEvents * This);
        
        DECLSPEC_XFGVIRT(IComAppEvents, OnAppActivation)
        HRESULT ( STDMETHODCALLTYPE *OnAppActivation )( 
            __RPC__in IComAppEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComAppEvents, OnAppShutdown)
        HRESULT ( STDMETHODCALLTYPE *OnAppShutdown )( 
            __RPC__in IComAppEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComAppEvents, OnAppForceShutdown)
        HRESULT ( STDMETHODCALLTYPE *OnAppForceShutdown )( 
            __RPC__in IComAppEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        END_INTERFACE
    } IComAppEventsVtbl;

    interface IComAppEvents
    {
        CONST_VTBL struct IComAppEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComAppEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComAppEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComAppEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComAppEvents_OnAppActivation(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnAppActivation(This,pInfo,guidApp) ) 

#define IComAppEvents_OnAppShutdown(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnAppShutdown(This,pInfo,guidApp) ) 

#define IComAppEvents_OnAppForceShutdown(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnAppForceShutdown(This,pInfo,guidApp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComAppEvents_INTERFACE_DEFINED__ */


#ifndef __IComInstanceEvents_INTERFACE_DEFINED__
#define __IComInstanceEvents_INTERFACE_DEFINED__

/* interface IComInstanceEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComInstanceEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A7-2E50-11d2-98A5-00C04F8EE1C4")
    IComInstanceEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjectCreate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjectDestroy( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComInstanceEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComInstanceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComInstanceEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComInstanceEvents * This);
        
        DECLSPEC_XFGVIRT(IComInstanceEvents, OnObjectCreate)
        HRESULT ( STDMETHODCALLTYPE *OnObjectCreate )( 
            __RPC__in IComInstanceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID);
        
        DECLSPEC_XFGVIRT(IComInstanceEvents, OnObjectDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnObjectDestroy )( 
            __RPC__in IComInstanceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        END_INTERFACE
    } IComInstanceEventsVtbl;

    interface IComInstanceEvents
    {
        CONST_VTBL struct IComInstanceEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComInstanceEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComInstanceEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComInstanceEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComInstanceEvents_OnObjectCreate(This,pInfo,guidActivity,clsid,tsid,CtxtID,ObjectID)	\
    ( (This)->lpVtbl -> OnObjectCreate(This,pInfo,guidActivity,clsid,tsid,CtxtID,ObjectID) ) 

#define IComInstanceEvents_OnObjectDestroy(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnObjectDestroy(This,pInfo,CtxtID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComInstanceEvents_INTERFACE_DEFINED__ */


#ifndef __IComTransactionEvents_INTERFACE_DEFINED__
#define __IComTransactionEvents_INTERFACE_DEFINED__

/* interface IComTransactionEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTransactionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A8-2E50-11d2-98A5-00C04F8EE1C4")
    IComTransactionEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTransactionStart( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ BOOL fRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionPrepare( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ BOOL fVoteYes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionAbort( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionCommit( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTransactionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTransactionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTransactionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTransactionEvents * This);
        
        DECLSPEC_XFGVIRT(IComTransactionEvents, OnTransactionStart)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionStart )( 
            __RPC__in IComTransactionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ BOOL fRoot);
        
        DECLSPEC_XFGVIRT(IComTransactionEvents, OnTransactionPrepare)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionPrepare )( 
            __RPC__in IComTransactionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ BOOL fVoteYes);
        
        DECLSPEC_XFGVIRT(IComTransactionEvents, OnTransactionAbort)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionAbort )( 
            __RPC__in IComTransactionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx);
        
        DECLSPEC_XFGVIRT(IComTransactionEvents, OnTransactionCommit)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionCommit )( 
            __RPC__in IComTransactionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx);
        
        END_INTERFACE
    } IComTransactionEventsVtbl;

    interface IComTransactionEvents
    {
        CONST_VTBL struct IComTransactionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTransactionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTransactionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTransactionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTransactionEvents_OnTransactionStart(This,pInfo,guidTx,tsid,fRoot)	\
    ( (This)->lpVtbl -> OnTransactionStart(This,pInfo,guidTx,tsid,fRoot) ) 

#define IComTransactionEvents_OnTransactionPrepare(This,pInfo,guidTx,fVoteYes)	\
    ( (This)->lpVtbl -> OnTransactionPrepare(This,pInfo,guidTx,fVoteYes) ) 

#define IComTransactionEvents_OnTransactionAbort(This,pInfo,guidTx)	\
    ( (This)->lpVtbl -> OnTransactionAbort(This,pInfo,guidTx) ) 

#define IComTransactionEvents_OnTransactionCommit(This,pInfo,guidTx)	\
    ( (This)->lpVtbl -> OnTransactionCommit(This,pInfo,guidTx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTransactionEvents_INTERFACE_DEFINED__ */


#ifndef __IComMethodEvents_INTERFACE_DEFINED__
#define __IComMethodEvents_INTERFACE_DEFINED__

/* interface IComMethodEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComMethodEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130A9-2E50-11d2-98A5-00C04F8EE1C4")
    IComMethodEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnMethodCall( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMethodReturn( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth,
            /* [in] */ HRESULT hresult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMethodException( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComMethodEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComMethodEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComMethodEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComMethodEvents * This);
        
        DECLSPEC_XFGVIRT(IComMethodEvents, OnMethodCall)
        HRESULT ( STDMETHODCALLTYPE *OnMethodCall )( 
            __RPC__in IComMethodEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth);
        
        DECLSPEC_XFGVIRT(IComMethodEvents, OnMethodReturn)
        HRESULT ( STDMETHODCALLTYPE *OnMethodReturn )( 
            __RPC__in IComMethodEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth,
            /* [in] */ HRESULT hresult);
        
        DECLSPEC_XFGVIRT(IComMethodEvents, OnMethodException)
        HRESULT ( STDMETHODCALLTYPE *OnMethodException )( 
            __RPC__in IComMethodEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ ULONG iMeth);
        
        END_INTERFACE
    } IComMethodEventsVtbl;

    interface IComMethodEvents
    {
        CONST_VTBL struct IComMethodEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComMethodEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComMethodEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComMethodEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComMethodEvents_OnMethodCall(This,pInfo,oid,guidCid,guidRid,iMeth)	\
    ( (This)->lpVtbl -> OnMethodCall(This,pInfo,oid,guidCid,guidRid,iMeth) ) 

#define IComMethodEvents_OnMethodReturn(This,pInfo,oid,guidCid,guidRid,iMeth,hresult)	\
    ( (This)->lpVtbl -> OnMethodReturn(This,pInfo,oid,guidCid,guidRid,iMeth,hresult) ) 

#define IComMethodEvents_OnMethodException(This,pInfo,oid,guidCid,guidRid,iMeth)	\
    ( (This)->lpVtbl -> OnMethodException(This,pInfo,oid,guidCid,guidRid,iMeth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComMethodEvents_INTERFACE_DEFINED__ */


#ifndef __IComObjectEvents_INTERFACE_DEFINED__
#define __IComObjectEvents_INTERFACE_DEFINED__

/* interface IComObjectEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AA-2E50-11d2-98A5-00C04F8EE1C4")
    IComObjectEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjectActivate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjectDeactivate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDisableCommit( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEnableCommit( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSetComplete( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSetAbort( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectEvents * This);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnObjectActivate)
        HRESULT ( STDMETHODCALLTYPE *OnObjectActivate )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnObjectDeactivate)
        HRESULT ( STDMETHODCALLTYPE *OnObjectDeactivate )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnDisableCommit)
        HRESULT ( STDMETHODCALLTYPE *OnDisableCommit )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnEnableCommit)
        HRESULT ( STDMETHODCALLTYPE *OnEnableCommit )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnSetComplete)
        HRESULT ( STDMETHODCALLTYPE *OnSetComplete )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        DECLSPEC_XFGVIRT(IComObjectEvents, OnSetAbort)
        HRESULT ( STDMETHODCALLTYPE *OnSetAbort )( 
            __RPC__in IComObjectEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        END_INTERFACE
    } IComObjectEventsVtbl;

    interface IComObjectEvents
    {
        CONST_VTBL struct IComObjectEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectEvents_OnObjectActivate(This,pInfo,CtxtID,ObjectID)	\
    ( (This)->lpVtbl -> OnObjectActivate(This,pInfo,CtxtID,ObjectID) ) 

#define IComObjectEvents_OnObjectDeactivate(This,pInfo,CtxtID,ObjectID)	\
    ( (This)->lpVtbl -> OnObjectDeactivate(This,pInfo,CtxtID,ObjectID) ) 

#define IComObjectEvents_OnDisableCommit(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnDisableCommit(This,pInfo,CtxtID) ) 

#define IComObjectEvents_OnEnableCommit(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnEnableCommit(This,pInfo,CtxtID) ) 

#define IComObjectEvents_OnSetComplete(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnSetComplete(This,pInfo,CtxtID) ) 

#define IComObjectEvents_OnSetAbort(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnSetAbort(This,pInfo,CtxtID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectEvents_INTERFACE_DEFINED__ */


#ifndef __IComResourceEvents_INTERFACE_DEFINED__
#define __IComResourceEvents_INTERFACE_DEFINED__

/* interface IComResourceEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComResourceEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AB-2E50-11d2-98A5-00C04F8EE1C4")
    IComResourceEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnResourceCreate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResourceAllocate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted,
            /* [in] */ DWORD NumRated,
            /* [in] */ DWORD Rating) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResourceRecycle( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResourceDestroy( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResourceTrack( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComResourceEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComResourceEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComResourceEvents * This);
        
        DECLSPEC_XFGVIRT(IComResourceEvents, OnResourceCreate)
        HRESULT ( STDMETHODCALLTYPE *OnResourceCreate )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted);
        
        DECLSPEC_XFGVIRT(IComResourceEvents, OnResourceAllocate)
        HRESULT ( STDMETHODCALLTYPE *OnResourceAllocate )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted,
            /* [in] */ DWORD NumRated,
            /* [in] */ DWORD Rating);
        
        DECLSPEC_XFGVIRT(IComResourceEvents, OnResourceRecycle)
        HRESULT ( STDMETHODCALLTYPE *OnResourceRecycle )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId);
        
        DECLSPEC_XFGVIRT(IComResourceEvents, OnResourceDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnResourceDestroy )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId);
        
        DECLSPEC_XFGVIRT(IComResourceEvents, OnResourceTrack)
        HRESULT ( STDMETHODCALLTYPE *OnResourceTrack )( 
            __RPC__in IComResourceEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in LPCOLESTR pszType,
            /* [in] */ ULONG64 resId,
            /* [in] */ BOOL enlisted);
        
        END_INTERFACE
    } IComResourceEventsVtbl;

    interface IComResourceEvents
    {
        CONST_VTBL struct IComResourceEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComResourceEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComResourceEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComResourceEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComResourceEvents_OnResourceCreate(This,pInfo,ObjectID,pszType,resId,enlisted)	\
    ( (This)->lpVtbl -> OnResourceCreate(This,pInfo,ObjectID,pszType,resId,enlisted) ) 

#define IComResourceEvents_OnResourceAllocate(This,pInfo,ObjectID,pszType,resId,enlisted,NumRated,Rating)	\
    ( (This)->lpVtbl -> OnResourceAllocate(This,pInfo,ObjectID,pszType,resId,enlisted,NumRated,Rating) ) 

#define IComResourceEvents_OnResourceRecycle(This,pInfo,ObjectID,pszType,resId)	\
    ( (This)->lpVtbl -> OnResourceRecycle(This,pInfo,ObjectID,pszType,resId) ) 

#define IComResourceEvents_OnResourceDestroy(This,pInfo,ObjectID,hr,pszType,resId)	\
    ( (This)->lpVtbl -> OnResourceDestroy(This,pInfo,ObjectID,hr,pszType,resId) ) 

#define IComResourceEvents_OnResourceTrack(This,pInfo,ObjectID,pszType,resId,enlisted)	\
    ( (This)->lpVtbl -> OnResourceTrack(This,pInfo,ObjectID,pszType,resId,enlisted) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComResourceEvents_INTERFACE_DEFINED__ */


#ifndef __IComSecurityEvents_INTERFACE_DEFINED__
#define __IComSecurityEvents_INTERFACE_DEFINED__

/* interface IComSecurityEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComSecurityEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AC-2E50-11d2-98A5-00C04F8EE1C4")
    IComSecurityEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnAuthenticate( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            ULONG64 ObjectID,
            __RPC__in REFGUID guidIID,
            ULONG iMeth,
            ULONG cbByteOrig,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteOrig) BYTE *pSidOriginalUser,
            ULONG cbByteCur,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteCur) BYTE *pSidCurrentUser,
            BOOL bCurrentUserInpersonatingInProc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAuthenticateFail( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            ULONG64 ObjectID,
            __RPC__in REFGUID guidIID,
            ULONG iMeth,
            ULONG cbByteOrig,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteOrig) BYTE *pSidOriginalUser,
            ULONG cbByteCur,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteCur) BYTE *pSidCurrentUser,
            BOOL bCurrentUserInpersonatingInProc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComSecurityEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComSecurityEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComSecurityEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComSecurityEvents * This);
        
        DECLSPEC_XFGVIRT(IComSecurityEvents, OnAuthenticate)
        HRESULT ( STDMETHODCALLTYPE *OnAuthenticate )( 
            __RPC__in IComSecurityEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            ULONG64 ObjectID,
            __RPC__in REFGUID guidIID,
            ULONG iMeth,
            ULONG cbByteOrig,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteOrig) BYTE *pSidOriginalUser,
            ULONG cbByteCur,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteCur) BYTE *pSidCurrentUser,
            BOOL bCurrentUserInpersonatingInProc);
        
        DECLSPEC_XFGVIRT(IComSecurityEvents, OnAuthenticateFail)
        HRESULT ( STDMETHODCALLTYPE *OnAuthenticateFail )( 
            __RPC__in IComSecurityEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            ULONG64 ObjectID,
            __RPC__in REFGUID guidIID,
            ULONG iMeth,
            ULONG cbByteOrig,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteOrig) BYTE *pSidOriginalUser,
            ULONG cbByteCur,
            /* [size_is][in] */ __RPC__in_ecount_full(cbByteCur) BYTE *pSidCurrentUser,
            BOOL bCurrentUserInpersonatingInProc);
        
        END_INTERFACE
    } IComSecurityEventsVtbl;

    interface IComSecurityEvents
    {
        CONST_VTBL struct IComSecurityEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComSecurityEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComSecurityEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComSecurityEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComSecurityEvents_OnAuthenticate(This,pInfo,guidActivity,ObjectID,guidIID,iMeth,cbByteOrig,pSidOriginalUser,cbByteCur,pSidCurrentUser,bCurrentUserInpersonatingInProc)	\
    ( (This)->lpVtbl -> OnAuthenticate(This,pInfo,guidActivity,ObjectID,guidIID,iMeth,cbByteOrig,pSidOriginalUser,cbByteCur,pSidCurrentUser,bCurrentUserInpersonatingInProc) ) 

#define IComSecurityEvents_OnAuthenticateFail(This,pInfo,guidActivity,ObjectID,guidIID,iMeth,cbByteOrig,pSidOriginalUser,cbByteCur,pSidCurrentUser,bCurrentUserInpersonatingInProc)	\
    ( (This)->lpVtbl -> OnAuthenticateFail(This,pInfo,guidActivity,ObjectID,guidIID,iMeth,cbByteOrig,pSidOriginalUser,cbByteCur,pSidCurrentUser,bCurrentUserInpersonatingInProc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComSecurityEvents_INTERFACE_DEFINED__ */


#ifndef __IComObjectPoolEvents_INTERFACE_DEFINED__
#define __IComObjectPoolEvents_INTERFACE_DEFINED__

/* interface IComObjectPoolEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectPoolEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AD-2E50-11d2-98A5-00C04F8EE1C4")
    IComObjectPoolEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolPutObject( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            int nReason,
            DWORD dwAvailable,
            ULONG64 oid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolGetObject( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            DWORD dwAvailable,
            ULONG64 oid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolRecycleToTx( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidTx,
            ULONG64 objid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolGetFromTx( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidTx,
            ULONG64 objid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectPoolEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectPoolEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectPoolEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectPoolEvents * This);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents, OnObjPoolPutObject)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolPutObject )( 
            __RPC__in IComObjectPoolEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            int nReason,
            DWORD dwAvailable,
            ULONG64 oid);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents, OnObjPoolGetObject)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolGetObject )( 
            __RPC__in IComObjectPoolEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            DWORD dwAvailable,
            ULONG64 oid);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents, OnObjPoolRecycleToTx)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolRecycleToTx )( 
            __RPC__in IComObjectPoolEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidTx,
            ULONG64 objid);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents, OnObjPoolGetFromTx)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolGetFromTx )( 
            __RPC__in IComObjectPoolEvents * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidActivity,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidTx,
            ULONG64 objid);
        
        END_INTERFACE
    } IComObjectPoolEventsVtbl;

    interface IComObjectPoolEvents
    {
        CONST_VTBL struct IComObjectPoolEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectPoolEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectPoolEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectPoolEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectPoolEvents_OnObjPoolPutObject(This,pInfo,guidObject,nReason,dwAvailable,oid)	\
    ( (This)->lpVtbl -> OnObjPoolPutObject(This,pInfo,guidObject,nReason,dwAvailable,oid) ) 

#define IComObjectPoolEvents_OnObjPoolGetObject(This,pInfo,guidActivity,guidObject,dwAvailable,oid)	\
    ( (This)->lpVtbl -> OnObjPoolGetObject(This,pInfo,guidActivity,guidObject,dwAvailable,oid) ) 

#define IComObjectPoolEvents_OnObjPoolRecycleToTx(This,pInfo,guidActivity,guidObject,guidTx,objid)	\
    ( (This)->lpVtbl -> OnObjPoolRecycleToTx(This,pInfo,guidActivity,guidObject,guidTx,objid) ) 

#define IComObjectPoolEvents_OnObjPoolGetFromTx(This,pInfo,guidActivity,guidObject,guidTx,objid)	\
    ( (This)->lpVtbl -> OnObjPoolGetFromTx(This,pInfo,guidActivity,guidObject,guidTx,objid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectPoolEvents_INTERFACE_DEFINED__ */


#ifndef __IComObjectPoolEvents2_INTERFACE_DEFINED__
#define __IComObjectPoolEvents2_INTERFACE_DEFINED__

/* interface IComObjectPoolEvents2 */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectPoolEvents2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AE-2E50-11d2-98A5-00C04F8EE1C4")
    IComObjectPoolEvents2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolCreateObject( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwObjsCreated,
            ULONG64 oid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolDestroyObject( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwObjsCreated,
            ULONG64 oid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolCreateDecision( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            DWORD dwThreadsWaiting,
            DWORD dwAvail,
            DWORD dwCreated,
            DWORD dwMin,
            DWORD dwMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolTimeout( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidActivity,
            DWORD dwTimeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolCreatePool( 
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwMin,
            DWORD dwMax,
            DWORD dwTimeout) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectPoolEvents2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectPoolEvents2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectPoolEvents2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectPoolEvents2 * This);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents2, OnObjPoolCreateObject)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolCreateObject )( 
            __RPC__in IComObjectPoolEvents2 * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwObjsCreated,
            ULONG64 oid);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents2, OnObjPoolDestroyObject)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolDestroyObject )( 
            __RPC__in IComObjectPoolEvents2 * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwObjsCreated,
            ULONG64 oid);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents2, OnObjPoolCreateDecision)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolCreateDecision )( 
            __RPC__in IComObjectPoolEvents2 * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            DWORD dwThreadsWaiting,
            DWORD dwAvail,
            DWORD dwCreated,
            DWORD dwMin,
            DWORD dwMax);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents2, OnObjPoolTimeout)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolTimeout )( 
            __RPC__in IComObjectPoolEvents2 * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            __RPC__in REFGUID guidActivity,
            DWORD dwTimeout);
        
        DECLSPEC_XFGVIRT(IComObjectPoolEvents2, OnObjPoolCreatePool)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolCreatePool )( 
            __RPC__in IComObjectPoolEvents2 * This,
            __RPC__in COMSVCSEVENTINFO *pInfo,
            __RPC__in REFGUID guidObject,
            DWORD dwMin,
            DWORD dwMax,
            DWORD dwTimeout);
        
        END_INTERFACE
    } IComObjectPoolEvents2Vtbl;

    interface IComObjectPoolEvents2
    {
        CONST_VTBL struct IComObjectPoolEvents2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectPoolEvents2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectPoolEvents2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectPoolEvents2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectPoolEvents2_OnObjPoolCreateObject(This,pInfo,guidObject,dwObjsCreated,oid)	\
    ( (This)->lpVtbl -> OnObjPoolCreateObject(This,pInfo,guidObject,dwObjsCreated,oid) ) 

#define IComObjectPoolEvents2_OnObjPoolDestroyObject(This,pInfo,guidObject,dwObjsCreated,oid)	\
    ( (This)->lpVtbl -> OnObjPoolDestroyObject(This,pInfo,guidObject,dwObjsCreated,oid) ) 

#define IComObjectPoolEvents2_OnObjPoolCreateDecision(This,pInfo,dwThreadsWaiting,dwAvail,dwCreated,dwMin,dwMax)	\
    ( (This)->lpVtbl -> OnObjPoolCreateDecision(This,pInfo,dwThreadsWaiting,dwAvail,dwCreated,dwMin,dwMax) ) 

#define IComObjectPoolEvents2_OnObjPoolTimeout(This,pInfo,guidObject,guidActivity,dwTimeout)	\
    ( (This)->lpVtbl -> OnObjPoolTimeout(This,pInfo,guidObject,guidActivity,dwTimeout) ) 

#define IComObjectPoolEvents2_OnObjPoolCreatePool(This,pInfo,guidObject,dwMin,dwMax,dwTimeout)	\
    ( (This)->lpVtbl -> OnObjPoolCreatePool(This,pInfo,guidObject,dwMin,dwMax,dwTimeout) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectPoolEvents2_INTERFACE_DEFINED__ */


#ifndef __IComObjectConstructionEvents_INTERFACE_DEFINED__
#define __IComObjectConstructionEvents_INTERFACE_DEFINED__

/* interface IComObjectConstructionEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectConstructionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130AF-2E50-11d2-98A5-00C04F8EE1C4")
    IComObjectConstructionEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjectConstruct( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in LPCOLESTR sConstructString,
            /* [in] */ ULONG64 oid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectConstructionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectConstructionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectConstructionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectConstructionEvents * This);
        
        DECLSPEC_XFGVIRT(IComObjectConstructionEvents, OnObjectConstruct)
        HRESULT ( STDMETHODCALLTYPE *OnObjectConstruct )( 
            __RPC__in IComObjectConstructionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in LPCOLESTR sConstructString,
            /* [in] */ ULONG64 oid);
        
        END_INTERFACE
    } IComObjectConstructionEventsVtbl;

    interface IComObjectConstructionEvents
    {
        CONST_VTBL struct IComObjectConstructionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectConstructionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectConstructionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectConstructionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectConstructionEvents_OnObjectConstruct(This,pInfo,guidObject,sConstructString,oid)	\
    ( (This)->lpVtbl -> OnObjectConstruct(This,pInfo,guidObject,sConstructString,oid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectConstructionEvents_INTERFACE_DEFINED__ */


#ifndef __IComActivityEvents_INTERFACE_DEFINED__
#define __IComActivityEvents_INTERFACE_DEFINED__

/* interface IComActivityEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComActivityEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B0-2E50-11d2-98A5-00C04F8EE1C4")
    IComActivityEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnActivityCreate( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityDestroy( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityEnter( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidEntered,
            /* [in] */ DWORD dwThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityTimeout( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidEntered,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTimeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityReenter( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwCallDepth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityLeave( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidLeft) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnActivityLeaveSame( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ DWORD dwCallDepth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComActivityEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComActivityEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComActivityEvents * This);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityCreate)
        HRESULT ( STDMETHODCALLTYPE *OnActivityCreate )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnActivityDestroy )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityEnter)
        HRESULT ( STDMETHODCALLTYPE *OnActivityEnter )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidEntered,
            /* [in] */ DWORD dwThread);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityTimeout)
        HRESULT ( STDMETHODCALLTYPE *OnActivityTimeout )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidEntered,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwTimeout);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityReenter)
        HRESULT ( STDMETHODCALLTYPE *OnActivityReenter )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ DWORD dwThread,
            /* [in] */ DWORD dwCallDepth);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityLeave)
        HRESULT ( STDMETHODCALLTYPE *OnActivityLeave )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ __RPC__in REFGUID guidLeft);
        
        DECLSPEC_XFGVIRT(IComActivityEvents, OnActivityLeaveSame)
        HRESULT ( STDMETHODCALLTYPE *OnActivityLeaveSame )( 
            __RPC__in IComActivityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidCurrent,
            /* [in] */ DWORD dwCallDepth);
        
        END_INTERFACE
    } IComActivityEventsVtbl;

    interface IComActivityEvents
    {
        CONST_VTBL struct IComActivityEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComActivityEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComActivityEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComActivityEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComActivityEvents_OnActivityCreate(This,pInfo,guidActivity)	\
    ( (This)->lpVtbl -> OnActivityCreate(This,pInfo,guidActivity) ) 

#define IComActivityEvents_OnActivityDestroy(This,pInfo,guidActivity)	\
    ( (This)->lpVtbl -> OnActivityDestroy(This,pInfo,guidActivity) ) 

#define IComActivityEvents_OnActivityEnter(This,pInfo,guidCurrent,guidEntered,dwThread)	\
    ( (This)->lpVtbl -> OnActivityEnter(This,pInfo,guidCurrent,guidEntered,dwThread) ) 

#define IComActivityEvents_OnActivityTimeout(This,pInfo,guidCurrent,guidEntered,dwThread,dwTimeout)	\
    ( (This)->lpVtbl -> OnActivityTimeout(This,pInfo,guidCurrent,guidEntered,dwThread,dwTimeout) ) 

#define IComActivityEvents_OnActivityReenter(This,pInfo,guidCurrent,dwThread,dwCallDepth)	\
    ( (This)->lpVtbl -> OnActivityReenter(This,pInfo,guidCurrent,dwThread,dwCallDepth) ) 

#define IComActivityEvents_OnActivityLeave(This,pInfo,guidCurrent,guidLeft)	\
    ( (This)->lpVtbl -> OnActivityLeave(This,pInfo,guidCurrent,guidLeft) ) 

#define IComActivityEvents_OnActivityLeaveSame(This,pInfo,guidCurrent,dwCallDepth)	\
    ( (This)->lpVtbl -> OnActivityLeaveSame(This,pInfo,guidCurrent,dwCallDepth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComActivityEvents_INTERFACE_DEFINED__ */


#ifndef __IComIdentityEvents_INTERFACE_DEFINED__
#define __IComIdentityEvents_INTERFACE_DEFINED__

/* interface IComIdentityEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComIdentityEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B1-2E50-11d2-98A5-00C04F8EE1C4")
    IComIdentityEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnIISRequestInfo( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjId,
            /* [in] */ __RPC__in LPCOLESTR pszClientIP,
            /* [in] */ __RPC__in LPCOLESTR pszServerIP,
            /* [in] */ __RPC__in LPCOLESTR pszURL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComIdentityEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComIdentityEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComIdentityEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComIdentityEvents * This);
        
        DECLSPEC_XFGVIRT(IComIdentityEvents, OnIISRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *OnIISRequestInfo )( 
            __RPC__in IComIdentityEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 ObjId,
            /* [in] */ __RPC__in LPCOLESTR pszClientIP,
            /* [in] */ __RPC__in LPCOLESTR pszServerIP,
            /* [in] */ __RPC__in LPCOLESTR pszURL);
        
        END_INTERFACE
    } IComIdentityEventsVtbl;

    interface IComIdentityEvents
    {
        CONST_VTBL struct IComIdentityEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComIdentityEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComIdentityEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComIdentityEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComIdentityEvents_OnIISRequestInfo(This,pInfo,ObjId,pszClientIP,pszServerIP,pszURL)	\
    ( (This)->lpVtbl -> OnIISRequestInfo(This,pInfo,ObjId,pszClientIP,pszServerIP,pszURL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComIdentityEvents_INTERFACE_DEFINED__ */


#ifndef __IComQCEvents_INTERFACE_DEFINED__
#define __IComQCEvents_INTERFACE_DEFINED__

/* interface IComQCEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComQCEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B2-2E50-11d2-98A5-00C04F8EE1C4")
    IComQCEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnQCRecord( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in_ecount_full(60) WCHAR szQueue[ 60 ],
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT msmqhr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCQueueOpen( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in_ecount_full(60) WCHAR szQueue[ 60 ],
            /* [in] */ ULONG64 QueueID,
            /* [in] */ HRESULT hr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCReceive( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 QueueID,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT hr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCReceiveFail( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 QueueID,
            /* [in] */ HRESULT msmqhr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCMoveToReTryQueue( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ ULONG RetryIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCMoveToDeadQueue( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnQCPlayback( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT hr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComQCEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComQCEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComQCEvents * This);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCRecord)
        HRESULT ( STDMETHODCALLTYPE *OnQCRecord )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in_ecount_full(60) WCHAR szQueue[ 60 ],
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT msmqhr);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCQueueOpen)
        HRESULT ( STDMETHODCALLTYPE *OnQCQueueOpen )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in_ecount_full(60) WCHAR szQueue[ 60 ],
            /* [in] */ ULONG64 QueueID,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCReceive)
        HRESULT ( STDMETHODCALLTYPE *OnQCReceive )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 QueueID,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCReceiveFail)
        HRESULT ( STDMETHODCALLTYPE *OnQCReceiveFail )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 QueueID,
            /* [in] */ HRESULT msmqhr);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCMoveToReTryQueue)
        HRESULT ( STDMETHODCALLTYPE *OnQCMoveToReTryQueue )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ ULONG RetryIndex);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCMoveToDeadQueue)
        HRESULT ( STDMETHODCALLTYPE *OnQCMoveToDeadQueue )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId);
        
        DECLSPEC_XFGVIRT(IComQCEvents, OnQCPlayback)
        HRESULT ( STDMETHODCALLTYPE *OnQCPlayback )( 
            __RPC__in IComQCEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in REFGUID guidMsgId,
            /* [in] */ __RPC__in REFGUID guidWorkFlowId,
            /* [in] */ HRESULT hr);
        
        END_INTERFACE
    } IComQCEventsVtbl;

    interface IComQCEvents
    {
        CONST_VTBL struct IComQCEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComQCEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComQCEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComQCEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComQCEvents_OnQCRecord(This,pInfo,objid,szQueue,guidMsgId,guidWorkFlowId,msmqhr)	\
    ( (This)->lpVtbl -> OnQCRecord(This,pInfo,objid,szQueue,guidMsgId,guidWorkFlowId,msmqhr) ) 

#define IComQCEvents_OnQCQueueOpen(This,pInfo,szQueue,QueueID,hr)	\
    ( (This)->lpVtbl -> OnQCQueueOpen(This,pInfo,szQueue,QueueID,hr) ) 

#define IComQCEvents_OnQCReceive(This,pInfo,QueueID,guidMsgId,guidWorkFlowId,hr)	\
    ( (This)->lpVtbl -> OnQCReceive(This,pInfo,QueueID,guidMsgId,guidWorkFlowId,hr) ) 

#define IComQCEvents_OnQCReceiveFail(This,pInfo,QueueID,msmqhr)	\
    ( (This)->lpVtbl -> OnQCReceiveFail(This,pInfo,QueueID,msmqhr) ) 

#define IComQCEvents_OnQCMoveToReTryQueue(This,pInfo,guidMsgId,guidWorkFlowId,RetryIndex)	\
    ( (This)->lpVtbl -> OnQCMoveToReTryQueue(This,pInfo,guidMsgId,guidWorkFlowId,RetryIndex) ) 

#define IComQCEvents_OnQCMoveToDeadQueue(This,pInfo,guidMsgId,guidWorkFlowId)	\
    ( (This)->lpVtbl -> OnQCMoveToDeadQueue(This,pInfo,guidMsgId,guidWorkFlowId) ) 

#define IComQCEvents_OnQCPlayback(This,pInfo,objid,guidMsgId,guidWorkFlowId,hr)	\
    ( (This)->lpVtbl -> OnQCPlayback(This,pInfo,objid,guidMsgId,guidWorkFlowId,hr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComQCEvents_INTERFACE_DEFINED__ */


#ifndef __IComExceptionEvents_INTERFACE_DEFINED__
#define __IComExceptionEvents_INTERFACE_DEFINED__

/* interface IComExceptionEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComExceptionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B3-2E50-11d2-98A5-00C04F8EE1C4")
    IComExceptionEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnExceptionUser( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG code,
            /* [in] */ ULONG64 address,
            /* [in] */ __RPC__in LPCOLESTR pszStackTrace) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComExceptionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComExceptionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComExceptionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComExceptionEvents * This);
        
        DECLSPEC_XFGVIRT(IComExceptionEvents, OnExceptionUser)
        HRESULT ( STDMETHODCALLTYPE *OnExceptionUser )( 
            __RPC__in IComExceptionEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG code,
            /* [in] */ ULONG64 address,
            /* [in] */ __RPC__in LPCOLESTR pszStackTrace);
        
        END_INTERFACE
    } IComExceptionEventsVtbl;

    interface IComExceptionEvents
    {
        CONST_VTBL struct IComExceptionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComExceptionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComExceptionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComExceptionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComExceptionEvents_OnExceptionUser(This,pInfo,code,address,pszStackTrace)	\
    ( (This)->lpVtbl -> OnExceptionUser(This,pInfo,code,address,pszStackTrace) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComExceptionEvents_INTERFACE_DEFINED__ */


#ifndef __ILBEvents_INTERFACE_DEFINED__
#define __ILBEvents_INTERFACE_DEFINED__

/* interface ILBEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_ILBEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B4-2E50-11d2-98A5-00C04F8EE1C4")
    ILBEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TargetUp( 
            __RPC__in BSTR bstrServerName,
            __RPC__in BSTR bstrClsidEng) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TargetDown( 
            __RPC__in BSTR bstrServerName,
            __RPC__in BSTR bstrClsidEng) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EngineDefined( 
            __RPC__in BSTR bstrPropName,
            __RPC__in VARIANT *varPropValue,
            __RPC__in BSTR bstrClsidEng) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILBEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILBEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILBEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILBEvents * This);
        
        DECLSPEC_XFGVIRT(ILBEvents, TargetUp)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TargetUp )( 
            __RPC__in ILBEvents * This,
            __RPC__in BSTR bstrServerName,
            __RPC__in BSTR bstrClsidEng);
        
        DECLSPEC_XFGVIRT(ILBEvents, TargetDown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TargetDown )( 
            __RPC__in ILBEvents * This,
            __RPC__in BSTR bstrServerName,
            __RPC__in BSTR bstrClsidEng);
        
        DECLSPEC_XFGVIRT(ILBEvents, EngineDefined)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EngineDefined )( 
            __RPC__in ILBEvents * This,
            __RPC__in BSTR bstrPropName,
            __RPC__in VARIANT *varPropValue,
            __RPC__in BSTR bstrClsidEng);
        
        END_INTERFACE
    } ILBEventsVtbl;

    interface ILBEvents
    {
        CONST_VTBL struct ILBEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILBEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILBEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILBEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILBEvents_TargetUp(This,bstrServerName,bstrClsidEng)	\
    ( (This)->lpVtbl -> TargetUp(This,bstrServerName,bstrClsidEng) ) 

#define ILBEvents_TargetDown(This,bstrServerName,bstrClsidEng)	\
    ( (This)->lpVtbl -> TargetDown(This,bstrServerName,bstrClsidEng) ) 

#define ILBEvents_EngineDefined(This,bstrPropName,varPropValue,bstrClsidEng)	\
    ( (This)->lpVtbl -> EngineDefined(This,bstrPropName,varPropValue,bstrClsidEng) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILBEvents_INTERFACE_DEFINED__ */


#ifndef __IComCRMEvents_INTERFACE_DEFINED__
#define __IComCRMEvents_INTERFACE_DEFINED__

/* interface IComCRMEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComCRMEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683130B5-2E50-11d2-98A5-00C04F8EE1C4")
    IComCRMEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCRMRecoveryStart( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMRecoveryDone( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMCheckpoint( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMBegin( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ GUID guidActivity,
            /* [in] */ GUID guidTx,
            /* [in] */ __RPC__in_ecount_full(64) WCHAR szProgIdCompensator[ 64 ],
            /* [in] */ __RPC__in_ecount_full(64) WCHAR szDescription[ 64 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMPrepare( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMCommit( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMAbort( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMIndoubt( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMDone( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMRelease( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMAnalyze( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ DWORD dwCrmRecordType,
            /* [in] */ DWORD dwRecordSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMWrite( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ BOOL fVariants,
            /* [in] */ DWORD dwRecordSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMForget( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMForce( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCRMDeliver( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ BOOL fVariants,
            /* [in] */ DWORD dwRecordSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComCRMEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComCRMEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComCRMEvents * This);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMRecoveryStart)
        HRESULT ( STDMETHODCALLTYPE *OnCRMRecoveryStart )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMRecoveryDone)
        HRESULT ( STDMETHODCALLTYPE *OnCRMRecoveryDone )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMCheckpoint)
        HRESULT ( STDMETHODCALLTYPE *OnCRMCheckpoint )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMBegin)
        HRESULT ( STDMETHODCALLTYPE *OnCRMBegin )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ GUID guidActivity,
            /* [in] */ GUID guidTx,
            /* [in] */ __RPC__in_ecount_full(64) WCHAR szProgIdCompensator[ 64 ],
            /* [in] */ __RPC__in_ecount_full(64) WCHAR szDescription[ 64 ]);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMPrepare)
        HRESULT ( STDMETHODCALLTYPE *OnCRMPrepare )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMCommit)
        HRESULT ( STDMETHODCALLTYPE *OnCRMCommit )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMAbort)
        HRESULT ( STDMETHODCALLTYPE *OnCRMAbort )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMIndoubt)
        HRESULT ( STDMETHODCALLTYPE *OnCRMIndoubt )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMDone)
        HRESULT ( STDMETHODCALLTYPE *OnCRMDone )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMRelease)
        HRESULT ( STDMETHODCALLTYPE *OnCRMRelease )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMAnalyze)
        HRESULT ( STDMETHODCALLTYPE *OnCRMAnalyze )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ DWORD dwCrmRecordType,
            /* [in] */ DWORD dwRecordSize);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMWrite)
        HRESULT ( STDMETHODCALLTYPE *OnCRMWrite )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ BOOL fVariants,
            /* [in] */ DWORD dwRecordSize);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMForget)
        HRESULT ( STDMETHODCALLTYPE *OnCRMForget )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMForce)
        HRESULT ( STDMETHODCALLTYPE *OnCRMForce )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID);
        
        DECLSPEC_XFGVIRT(IComCRMEvents, OnCRMDeliver)
        HRESULT ( STDMETHODCALLTYPE *OnCRMDeliver )( 
            __RPC__in IComCRMEvents * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidClerkCLSID,
            /* [in] */ BOOL fVariants,
            /* [in] */ DWORD dwRecordSize);
        
        END_INTERFACE
    } IComCRMEventsVtbl;

    interface IComCRMEvents
    {
        CONST_VTBL struct IComCRMEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComCRMEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComCRMEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComCRMEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComCRMEvents_OnCRMRecoveryStart(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnCRMRecoveryStart(This,pInfo,guidApp) ) 

#define IComCRMEvents_OnCRMRecoveryDone(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnCRMRecoveryDone(This,pInfo,guidApp) ) 

#define IComCRMEvents_OnCRMCheckpoint(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnCRMCheckpoint(This,pInfo,guidApp) ) 

#define IComCRMEvents_OnCRMBegin(This,pInfo,guidClerkCLSID,guidActivity,guidTx,szProgIdCompensator,szDescription)	\
    ( (This)->lpVtbl -> OnCRMBegin(This,pInfo,guidClerkCLSID,guidActivity,guidTx,szProgIdCompensator,szDescription) ) 

#define IComCRMEvents_OnCRMPrepare(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMPrepare(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMCommit(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMCommit(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMAbort(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMAbort(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMIndoubt(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMIndoubt(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMDone(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMDone(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMRelease(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMRelease(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMAnalyze(This,pInfo,guidClerkCLSID,dwCrmRecordType,dwRecordSize)	\
    ( (This)->lpVtbl -> OnCRMAnalyze(This,pInfo,guidClerkCLSID,dwCrmRecordType,dwRecordSize) ) 

#define IComCRMEvents_OnCRMWrite(This,pInfo,guidClerkCLSID,fVariants,dwRecordSize)	\
    ( (This)->lpVtbl -> OnCRMWrite(This,pInfo,guidClerkCLSID,fVariants,dwRecordSize) ) 

#define IComCRMEvents_OnCRMForget(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMForget(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMForce(This,pInfo,guidClerkCLSID)	\
    ( (This)->lpVtbl -> OnCRMForce(This,pInfo,guidClerkCLSID) ) 

#define IComCRMEvents_OnCRMDeliver(This,pInfo,guidClerkCLSID,fVariants,dwRecordSize)	\
    ( (This)->lpVtbl -> OnCRMDeliver(This,pInfo,guidClerkCLSID,fVariants,dwRecordSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComCRMEvents_INTERFACE_DEFINED__ */


#ifndef __IComMethod2Events_INTERFACE_DEFINED__
#define __IComMethod2Events_INTERFACE_DEFINED__

/* interface IComMethod2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComMethod2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FB388AAA-567D-4024-AF8E-6E93EE748573")
    IComMethod2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnMethodCall2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMethodReturn2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth,
            /* [in] */ HRESULT hresult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMethodException2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComMethod2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComMethod2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComMethod2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComMethod2Events * This);
        
        DECLSPEC_XFGVIRT(IComMethod2Events, OnMethodCall2)
        HRESULT ( STDMETHODCALLTYPE *OnMethodCall2 )( 
            __RPC__in IComMethod2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth);
        
        DECLSPEC_XFGVIRT(IComMethod2Events, OnMethodReturn2)
        HRESULT ( STDMETHODCALLTYPE *OnMethodReturn2 )( 
            __RPC__in IComMethod2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth,
            /* [in] */ HRESULT hresult);
        
        DECLSPEC_XFGVIRT(IComMethod2Events, OnMethodException2)
        HRESULT ( STDMETHODCALLTYPE *OnMethodException2 )( 
            __RPC__in IComMethod2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFCLSID guidCid,
            /* [in] */ __RPC__in REFIID guidRid,
            /* [in] */ DWORD dwThread,
            /* [in] */ ULONG iMeth);
        
        END_INTERFACE
    } IComMethod2EventsVtbl;

    interface IComMethod2Events
    {
        CONST_VTBL struct IComMethod2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComMethod2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComMethod2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComMethod2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComMethod2Events_OnMethodCall2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth)	\
    ( (This)->lpVtbl -> OnMethodCall2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth) ) 

#define IComMethod2Events_OnMethodReturn2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth,hresult)	\
    ( (This)->lpVtbl -> OnMethodReturn2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth,hresult) ) 

#define IComMethod2Events_OnMethodException2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth)	\
    ( (This)->lpVtbl -> OnMethodException2(This,pInfo,oid,guidCid,guidRid,dwThread,iMeth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComMethod2Events_INTERFACE_DEFINED__ */


#ifndef __IComTrackingInfoEvents_INTERFACE_DEFINED__
#define __IComTrackingInfoEvents_INTERFACE_DEFINED__

/* interface IComTrackingInfoEvents */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTrackingInfoEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4e6cdcc9-fb25-4fd5-9cc5-c9f4b6559cec")
    IComTrackingInfoEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnNewTrackingInfo( 
            /* [in] */ __RPC__in_opt IUnknown *pToplevelCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTrackingInfoEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTrackingInfoEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTrackingInfoEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTrackingInfoEvents * This);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoEvents, OnNewTrackingInfo)
        HRESULT ( STDMETHODCALLTYPE *OnNewTrackingInfo )( 
            __RPC__in IComTrackingInfoEvents * This,
            /* [in] */ __RPC__in_opt IUnknown *pToplevelCollection);
        
        END_INTERFACE
    } IComTrackingInfoEventsVtbl;

    interface IComTrackingInfoEvents
    {
        CONST_VTBL struct IComTrackingInfoEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTrackingInfoEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTrackingInfoEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTrackingInfoEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTrackingInfoEvents_OnNewTrackingInfo(This,pToplevelCollection)	\
    ( (This)->lpVtbl -> OnNewTrackingInfo(This,pToplevelCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTrackingInfoEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0034 */
/* [local] */ 

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_autosvcs_0000_0034_0001
    {
        TRKCOLL_PROCESSES	= 0,
        TRKCOLL_APPLICATIONS	= ( TRKCOLL_PROCESSES + 1 ) ,
        TRKCOLL_COMPONENTS	= ( TRKCOLL_APPLICATIONS + 1 ) 
    } 	TRACKING_COLL_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0034_v0_0_s_ifspec;

#ifndef __IComTrackingInfoCollection_INTERFACE_DEFINED__
#define __IComTrackingInfoCollection_INTERFACE_DEFINED__

/* interface IComTrackingInfoCollection */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTrackingInfoCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c266c677-c9ad-49ab-9fd9-d9661078588a")
    IComTrackingInfoCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Type( 
            /* [out] */ __RPC__out TRACKING_COLL_TYPE *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [out] */ __RPC__out ULONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ ULONG ulIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTrackingInfoCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTrackingInfoCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTrackingInfoCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTrackingInfoCollection * This);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoCollection, Type)
        HRESULT ( STDMETHODCALLTYPE *Type )( 
            __RPC__in IComTrackingInfoCollection * This,
            /* [out] */ __RPC__out TRACKING_COLL_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoCollection, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            __RPC__in IComTrackingInfoCollection * This,
            /* [out] */ __RPC__out ULONG *pCount);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoCollection, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IComTrackingInfoCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IComTrackingInfoCollectionVtbl;

    interface IComTrackingInfoCollection
    {
        CONST_VTBL struct IComTrackingInfoCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTrackingInfoCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTrackingInfoCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTrackingInfoCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTrackingInfoCollection_Type(This,pType)	\
    ( (This)->lpVtbl -> Type(This,pType) ) 

#define IComTrackingInfoCollection_Count(This,pCount)	\
    ( (This)->lpVtbl -> Count(This,pCount) ) 

#define IComTrackingInfoCollection_Item(This,ulIndex,riid,ppv)	\
    ( (This)->lpVtbl -> Item(This,ulIndex,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTrackingInfoCollection_INTERFACE_DEFINED__ */


#ifndef __IComTrackingInfoObject_INTERFACE_DEFINED__
#define __IComTrackingInfoObject_INTERFACE_DEFINED__

/* interface IComTrackingInfoObject */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTrackingInfoObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("116e42c5-d8b1-47bf-ab1e-c895ed3e2372")
    IComTrackingInfoObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ __RPC__in LPOLESTR szPropertyName,
            /* [out] */ __RPC__out VARIANT *pvarOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTrackingInfoObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTrackingInfoObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTrackingInfoObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTrackingInfoObject * This);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoObject, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IComTrackingInfoObject * This,
            /* [in] */ __RPC__in LPOLESTR szPropertyName,
            /* [out] */ __RPC__out VARIANT *pvarOut);
        
        END_INTERFACE
    } IComTrackingInfoObjectVtbl;

    interface IComTrackingInfoObject
    {
        CONST_VTBL struct IComTrackingInfoObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTrackingInfoObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTrackingInfoObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTrackingInfoObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTrackingInfoObject_GetValue(This,szPropertyName,pvarOut)	\
    ( (This)->lpVtbl -> GetValue(This,szPropertyName,pvarOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTrackingInfoObject_INTERFACE_DEFINED__ */


#ifndef __IComTrackingInfoProperties_INTERFACE_DEFINED__
#define __IComTrackingInfoProperties_INTERFACE_DEFINED__

/* interface IComTrackingInfoProperties */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTrackingInfoProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("789b42be-6f6b-443a-898e-67abf390aa14")
    IComTrackingInfoProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PropCount( 
            /* [out] */ __RPC__out ULONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropName( 
            /* [in] */ ULONG ulIndex,
            /* [string][out] */ __RPC__deref_out_opt_string LPOLESTR *ppszPropName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTrackingInfoPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTrackingInfoProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTrackingInfoProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTrackingInfoProperties * This);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoProperties, PropCount)
        HRESULT ( STDMETHODCALLTYPE *PropCount )( 
            __RPC__in IComTrackingInfoProperties * This,
            /* [out] */ __RPC__out ULONG *pCount);
        
        DECLSPEC_XFGVIRT(IComTrackingInfoProperties, GetPropName)
        HRESULT ( STDMETHODCALLTYPE *GetPropName )( 
            __RPC__in IComTrackingInfoProperties * This,
            /* [in] */ ULONG ulIndex,
            /* [string][out] */ __RPC__deref_out_opt_string LPOLESTR *ppszPropName);
        
        END_INTERFACE
    } IComTrackingInfoPropertiesVtbl;

    interface IComTrackingInfoProperties
    {
        CONST_VTBL struct IComTrackingInfoPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTrackingInfoProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTrackingInfoProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTrackingInfoProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTrackingInfoProperties_PropCount(This,pCount)	\
    ( (This)->lpVtbl -> PropCount(This,pCount) ) 

#define IComTrackingInfoProperties_GetPropName(This,ulIndex,ppszPropName)	\
    ( (This)->lpVtbl -> GetPropName(This,ulIndex,ppszPropName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTrackingInfoProperties_INTERFACE_DEFINED__ */


#ifndef __IComApp2Events_INTERFACE_DEFINED__
#define __IComApp2Events_INTERFACE_DEFINED__

/* interface IComApp2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComApp2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1290BC1A-B219-418d-B078-5934DED08242")
    IComApp2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnAppActivation2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ GUID guidProcess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppShutdown2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppForceShutdown2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppPaused2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ BOOL bPaused) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAppRecycle2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ GUID guidProcess,
            /* [in] */ long lReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComApp2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComApp2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComApp2Events * This);
        
        DECLSPEC_XFGVIRT(IComApp2Events, OnAppActivation2)
        HRESULT ( STDMETHODCALLTYPE *OnAppActivation2 )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ GUID guidProcess);
        
        DECLSPEC_XFGVIRT(IComApp2Events, OnAppShutdown2)
        HRESULT ( STDMETHODCALLTYPE *OnAppShutdown2 )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComApp2Events, OnAppForceShutdown2)
        HRESULT ( STDMETHODCALLTYPE *OnAppForceShutdown2 )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp);
        
        DECLSPEC_XFGVIRT(IComApp2Events, OnAppPaused2)
        HRESULT ( STDMETHODCALLTYPE *OnAppPaused2 )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ BOOL bPaused);
        
        DECLSPEC_XFGVIRT(IComApp2Events, OnAppRecycle2)
        HRESULT ( STDMETHODCALLTYPE *OnAppRecycle2 )( 
            __RPC__in IComApp2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ GUID guidApp,
            /* [in] */ GUID guidProcess,
            /* [in] */ long lReason);
        
        END_INTERFACE
    } IComApp2EventsVtbl;

    interface IComApp2Events
    {
        CONST_VTBL struct IComApp2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComApp2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComApp2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComApp2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComApp2Events_OnAppActivation2(This,pInfo,guidApp,guidProcess)	\
    ( (This)->lpVtbl -> OnAppActivation2(This,pInfo,guidApp,guidProcess) ) 

#define IComApp2Events_OnAppShutdown2(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnAppShutdown2(This,pInfo,guidApp) ) 

#define IComApp2Events_OnAppForceShutdown2(This,pInfo,guidApp)	\
    ( (This)->lpVtbl -> OnAppForceShutdown2(This,pInfo,guidApp) ) 

#define IComApp2Events_OnAppPaused2(This,pInfo,guidApp,bPaused)	\
    ( (This)->lpVtbl -> OnAppPaused2(This,pInfo,guidApp,bPaused) ) 

#define IComApp2Events_OnAppRecycle2(This,pInfo,guidApp,guidProcess,lReason)	\
    ( (This)->lpVtbl -> OnAppRecycle2(This,pInfo,guidApp,guidProcess,lReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComApp2Events_INTERFACE_DEFINED__ */


#ifndef __IComTransaction2Events_INTERFACE_DEFINED__
#define __IComTransaction2Events_INTERFACE_DEFINED__

/* interface IComTransaction2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComTransaction2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A136F62A-2F94-4288-86E0-D8A1FA4C0299")
    IComTransaction2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTransactionStart2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ BOOL fRoot,
            /* [in] */ int nIsolationLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionPrepare2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ BOOL fVoteYes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionAbort2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTransactionCommit2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComTransaction2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComTransaction2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComTransaction2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComTransaction2Events * This);
        
        DECLSPEC_XFGVIRT(IComTransaction2Events, OnTransactionStart2)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionStart2 )( 
            __RPC__in IComTransaction2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ BOOL fRoot,
            /* [in] */ int nIsolationLevel);
        
        DECLSPEC_XFGVIRT(IComTransaction2Events, OnTransactionPrepare2)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionPrepare2 )( 
            __RPC__in IComTransaction2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ BOOL fVoteYes);
        
        DECLSPEC_XFGVIRT(IComTransaction2Events, OnTransactionAbort2)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionAbort2 )( 
            __RPC__in IComTransaction2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx);
        
        DECLSPEC_XFGVIRT(IComTransaction2Events, OnTransactionCommit2)
        HRESULT ( STDMETHODCALLTYPE *OnTransactionCommit2 )( 
            __RPC__in IComTransaction2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidTx);
        
        END_INTERFACE
    } IComTransaction2EventsVtbl;

    interface IComTransaction2Events
    {
        CONST_VTBL struct IComTransaction2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComTransaction2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComTransaction2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComTransaction2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComTransaction2Events_OnTransactionStart2(This,pInfo,guidTx,tsid,fRoot,nIsolationLevel)	\
    ( (This)->lpVtbl -> OnTransactionStart2(This,pInfo,guidTx,tsid,fRoot,nIsolationLevel) ) 

#define IComTransaction2Events_OnTransactionPrepare2(This,pInfo,guidTx,fVoteYes)	\
    ( (This)->lpVtbl -> OnTransactionPrepare2(This,pInfo,guidTx,fVoteYes) ) 

#define IComTransaction2Events_OnTransactionAbort2(This,pInfo,guidTx)	\
    ( (This)->lpVtbl -> OnTransactionAbort2(This,pInfo,guidTx) ) 

#define IComTransaction2Events_OnTransactionCommit2(This,pInfo,guidTx)	\
    ( (This)->lpVtbl -> OnTransactionCommit2(This,pInfo,guidTx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComTransaction2Events_INTERFACE_DEFINED__ */


#ifndef __IComInstance2Events_INTERFACE_DEFINED__
#define __IComInstance2Events_INTERFACE_DEFINED__

/* interface IComInstance2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComInstance2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20E3BF07-B506-4ad5-A50C-D2CA5B9C158E")
    IComInstance2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjectCreate2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in REFGUID guidPartition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjectDestroy2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComInstance2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComInstance2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComInstance2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComInstance2Events * This);
        
        DECLSPEC_XFGVIRT(IComInstance2Events, OnObjectCreate2)
        HRESULT ( STDMETHODCALLTYPE *OnObjectCreate2 )( 
            __RPC__in IComInstance2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID tsid,
            /* [in] */ ULONG64 CtxtID,
            /* [in] */ ULONG64 ObjectID,
            /* [in] */ __RPC__in REFGUID guidPartition);
        
        DECLSPEC_XFGVIRT(IComInstance2Events, OnObjectDestroy2)
        HRESULT ( STDMETHODCALLTYPE *OnObjectDestroy2 )( 
            __RPC__in IComInstance2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ ULONG64 CtxtID);
        
        END_INTERFACE
    } IComInstance2EventsVtbl;

    interface IComInstance2Events
    {
        CONST_VTBL struct IComInstance2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComInstance2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComInstance2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComInstance2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComInstance2Events_OnObjectCreate2(This,pInfo,guidActivity,clsid,tsid,CtxtID,ObjectID,guidPartition)	\
    ( (This)->lpVtbl -> OnObjectCreate2(This,pInfo,guidActivity,clsid,tsid,CtxtID,ObjectID,guidPartition) ) 

#define IComInstance2Events_OnObjectDestroy2(This,pInfo,CtxtID)	\
    ( (This)->lpVtbl -> OnObjectDestroy2(This,pInfo,CtxtID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComInstance2Events_INTERFACE_DEFINED__ */


#ifndef __IComObjectPool2Events_INTERFACE_DEFINED__
#define __IComObjectPool2Events_INTERFACE_DEFINED__

/* interface IComObjectPool2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectPool2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("65BF6534-85EA-4f64-8CF4-3D974B2AB1CF")
    IComObjectPool2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolPutObject2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ int nReason,
            /* [in] */ DWORD dwAvailable,
            /* [in] */ ULONG64 oid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolGetObject2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ DWORD dwAvailable,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFGUID guidPartition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolRecycleToTx2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ ULONG64 objid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjPoolGetFromTx2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in REFGUID guidPartition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectPool2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectPool2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectPool2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectPool2Events * This);
        
        DECLSPEC_XFGVIRT(IComObjectPool2Events, OnObjPoolPutObject2)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolPutObject2 )( 
            __RPC__in IComObjectPool2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ int nReason,
            /* [in] */ DWORD dwAvailable,
            /* [in] */ ULONG64 oid);
        
        DECLSPEC_XFGVIRT(IComObjectPool2Events, OnObjPoolGetObject2)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolGetObject2 )( 
            __RPC__in IComObjectPool2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ DWORD dwAvailable,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFGUID guidPartition);
        
        DECLSPEC_XFGVIRT(IComObjectPool2Events, OnObjPoolRecycleToTx2)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolRecycleToTx2 )( 
            __RPC__in IComObjectPool2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ ULONG64 objid);
        
        DECLSPEC_XFGVIRT(IComObjectPool2Events, OnObjPoolGetFromTx2)
        HRESULT ( STDMETHODCALLTYPE *OnObjPoolGetFromTx2 )( 
            __RPC__in IComObjectPool2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidActivity,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in REFGUID guidTx,
            /* [in] */ ULONG64 objid,
            /* [in] */ __RPC__in REFGUID guidPartition);
        
        END_INTERFACE
    } IComObjectPool2EventsVtbl;

    interface IComObjectPool2Events
    {
        CONST_VTBL struct IComObjectPool2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectPool2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectPool2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectPool2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectPool2Events_OnObjPoolPutObject2(This,pInfo,guidObject,nReason,dwAvailable,oid)	\
    ( (This)->lpVtbl -> OnObjPoolPutObject2(This,pInfo,guidObject,nReason,dwAvailable,oid) ) 

#define IComObjectPool2Events_OnObjPoolGetObject2(This,pInfo,guidActivity,guidObject,dwAvailable,oid,guidPartition)	\
    ( (This)->lpVtbl -> OnObjPoolGetObject2(This,pInfo,guidActivity,guidObject,dwAvailable,oid,guidPartition) ) 

#define IComObjectPool2Events_OnObjPoolRecycleToTx2(This,pInfo,guidActivity,guidObject,guidTx,objid)	\
    ( (This)->lpVtbl -> OnObjPoolRecycleToTx2(This,pInfo,guidActivity,guidObject,guidTx,objid) ) 

#define IComObjectPool2Events_OnObjPoolGetFromTx2(This,pInfo,guidActivity,guidObject,guidTx,objid,guidPartition)	\
    ( (This)->lpVtbl -> OnObjPoolGetFromTx2(This,pInfo,guidActivity,guidObject,guidTx,objid,guidPartition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectPool2Events_INTERFACE_DEFINED__ */


#ifndef __IComObjectConstruction2Events_INTERFACE_DEFINED__
#define __IComObjectConstruction2Events_INTERFACE_DEFINED__

/* interface IComObjectConstruction2Events */
/* [uuid][hidden][object] */ 


EXTERN_C const IID IID_IComObjectConstruction2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B5A7827-8DF2-45c0-8F6F-57EA1F856A9F")
    IComObjectConstruction2Events : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnObjectConstruct2( 
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in LPCOLESTR sConstructString,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFGUID guidPartition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComObjectConstruction2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComObjectConstruction2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComObjectConstruction2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComObjectConstruction2Events * This);
        
        DECLSPEC_XFGVIRT(IComObjectConstruction2Events, OnObjectConstruct2)
        HRESULT ( STDMETHODCALLTYPE *OnObjectConstruct2 )( 
            __RPC__in IComObjectConstruction2Events * This,
            /* [in] */ __RPC__in COMSVCSEVENTINFO *pInfo,
            /* [in] */ __RPC__in REFGUID guidObject,
            /* [in] */ __RPC__in LPCOLESTR sConstructString,
            /* [in] */ ULONG64 oid,
            /* [in] */ __RPC__in REFGUID guidPartition);
        
        END_INTERFACE
    } IComObjectConstruction2EventsVtbl;

    interface IComObjectConstruction2Events
    {
        CONST_VTBL struct IComObjectConstruction2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComObjectConstruction2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComObjectConstruction2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComObjectConstruction2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComObjectConstruction2Events_OnObjectConstruct2(This,pInfo,guidObject,sConstructString,oid,guidPartition)	\
    ( (This)->lpVtbl -> OnObjectConstruct2(This,pInfo,guidObject,sConstructString,oid,guidPartition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComObjectConstruction2Events_INTERFACE_DEFINED__ */


#ifndef __ISystemAppEventData_INTERFACE_DEFINED__
#define __ISystemAppEventData_INTERFACE_DEFINED__

/* interface ISystemAppEventData */
/* [unique][uuid][hidden][object] */ 


EXTERN_C const IID IID_ISystemAppEventData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6D48A3C-D5C5-49E7-8C74-99E4889ED52F")
    ISystemAppEventData : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Startup( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDataChanged( 
            /* [in] */ DWORD dwPID,
            /* [in] */ DWORD dwMask,
            /* [in] */ DWORD dwNumberSinks,
            /* [in] */ __RPC__in BSTR bstrDwMethodMask,
            /* [in] */ DWORD dwReason,
            /* [in] */ ULONG64 u64TraceHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemAppEventDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISystemAppEventData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISystemAppEventData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISystemAppEventData * This);
        
        DECLSPEC_XFGVIRT(ISystemAppEventData, Startup)
        HRESULT ( STDMETHODCALLTYPE *Startup )( 
            __RPC__in ISystemAppEventData * This);
        
        DECLSPEC_XFGVIRT(ISystemAppEventData, OnDataChanged)
        HRESULT ( STDMETHODCALLTYPE *OnDataChanged )( 
            __RPC__in ISystemAppEventData * This,
            /* [in] */ DWORD dwPID,
            /* [in] */ DWORD dwMask,
            /* [in] */ DWORD dwNumberSinks,
            /* [in] */ __RPC__in BSTR bstrDwMethodMask,
            /* [in] */ DWORD dwReason,
            /* [in] */ ULONG64 u64TraceHandle);
        
        END_INTERFACE
    } ISystemAppEventDataVtbl;

    interface ISystemAppEventData
    {
        CONST_VTBL struct ISystemAppEventDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemAppEventData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemAppEventData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemAppEventData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemAppEventData_Startup(This)	\
    ( (This)->lpVtbl -> Startup(This) ) 

#define ISystemAppEventData_OnDataChanged(This,dwPID,dwMask,dwNumberSinks,bstrDwMethodMask,dwReason,u64TraceHandle)	\
    ( (This)->lpVtbl -> OnDataChanged(This,dwPID,dwMask,dwNumberSinks,bstrDwMethodMask,dwReason,u64TraceHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemAppEventData_INTERFACE_DEFINED__ */


#ifndef __IMtsEvents_INTERFACE_DEFINED__
#define __IMtsEvents_INTERFACE_DEFINED__

/* interface IMtsEvents */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMtsEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BACEDF4D-74AB-11D0-B162-00AA00BA3258")
    IMtsEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PackageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PackageGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PostEvent( 
            /* [in] */ __RPC__in VARIANT *vEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FireEvents( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProcessID( 
            /* [retval][out] */ __RPC__out long *id) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMtsEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMtsEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMtsEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMtsEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMtsEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMtsEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMtsEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMtsEvents * This,
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
        
        DECLSPEC_XFGVIRT(IMtsEvents, get_PackageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PackageName )( 
            __RPC__in IMtsEvents * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IMtsEvents, get_PackageGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PackageGuid )( 
            __RPC__in IMtsEvents * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IMtsEvents, PostEvent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PostEvent )( 
            __RPC__in IMtsEvents * This,
            /* [in] */ __RPC__in VARIANT *vEvent);
        
        DECLSPEC_XFGVIRT(IMtsEvents, get_FireEvents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FireEvents )( 
            __RPC__in IMtsEvents * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IMtsEvents, GetProcessID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProcessID )( 
            __RPC__in IMtsEvents * This,
            /* [retval][out] */ __RPC__out long *id);
        
        END_INTERFACE
    } IMtsEventsVtbl;

    interface IMtsEvents
    {
        CONST_VTBL struct IMtsEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMtsEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMtsEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMtsEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMtsEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMtsEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMtsEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMtsEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMtsEvents_get_PackageName(This,pVal)	\
    ( (This)->lpVtbl -> get_PackageName(This,pVal) ) 

#define IMtsEvents_get_PackageGuid(This,pVal)	\
    ( (This)->lpVtbl -> get_PackageGuid(This,pVal) ) 

#define IMtsEvents_PostEvent(This,vEvent)	\
    ( (This)->lpVtbl -> PostEvent(This,vEvent) ) 

#define IMtsEvents_get_FireEvents(This,pVal)	\
    ( (This)->lpVtbl -> get_FireEvents(This,pVal) ) 

#define IMtsEvents_GetProcessID(This,id)	\
    ( (This)->lpVtbl -> GetProcessID(This,id) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMtsEvents_INTERFACE_DEFINED__ */


#ifndef __IMtsEventInfo_INTERFACE_DEFINED__
#define __IMtsEventInfo_INTERFACE_DEFINED__

/* interface IMtsEventInfo */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMtsEventInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D56C3DC1-8482-11d0-B170-00AA00BA3258")
    IMtsEventInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Names( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *sDisplayName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_EventID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *sGuidEventID) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *lCount) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ __RPC__in BSTR sKey,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMtsEventInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMtsEventInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMtsEventInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMtsEventInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMtsEventInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMtsEventInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMtsEventInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMtsEventInfo * This,
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
        
        DECLSPEC_XFGVIRT(IMtsEventInfo, get_Names)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Names )( 
            __RPC__in IMtsEventInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(IMtsEventInfo, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IMtsEventInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *sDisplayName);
        
        DECLSPEC_XFGVIRT(IMtsEventInfo, get_EventID)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventID )( 
            __RPC__in IMtsEventInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *sGuidEventID);
        
        DECLSPEC_XFGVIRT(IMtsEventInfo, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IMtsEventInfo * This,
            /* [retval][out] */ __RPC__out long *lCount);
        
        DECLSPEC_XFGVIRT(IMtsEventInfo, get_Value)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IMtsEventInfo * This,
            /* [in] */ __RPC__in BSTR sKey,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } IMtsEventInfoVtbl;

    interface IMtsEventInfo
    {
        CONST_VTBL struct IMtsEventInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMtsEventInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMtsEventInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMtsEventInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMtsEventInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMtsEventInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMtsEventInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMtsEventInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMtsEventInfo_get_Names(This,pUnk)	\
    ( (This)->lpVtbl -> get_Names(This,pUnk) ) 

#define IMtsEventInfo_get_DisplayName(This,sDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,sDisplayName) ) 

#define IMtsEventInfo_get_EventID(This,sGuidEventID)	\
    ( (This)->lpVtbl -> get_EventID(This,sGuidEventID) ) 

#define IMtsEventInfo_get_Count(This,lCount)	\
    ( (This)->lpVtbl -> get_Count(This,lCount) ) 

#define IMtsEventInfo_get_Value(This,sKey,pVal)	\
    ( (This)->lpVtbl -> get_Value(This,sKey,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMtsEventInfo_INTERFACE_DEFINED__ */


#ifndef __IMTSLocator_INTERFACE_DEFINED__
#define __IMTSLocator_INTERFACE_DEFINED__

/* interface IMTSLocator */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMTSLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D19B8BFD-7F88-11D0-B16E-00AA00BA3258")
    IMTSLocator : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEventDispatcher( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMTSLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMTSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMTSLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMTSLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMTSLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMTSLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMTSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMTSLocator * This,
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
        
        DECLSPEC_XFGVIRT(IMTSLocator, GetEventDispatcher)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventDispatcher )( 
            __RPC__in IMTSLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pUnk);
        
        END_INTERFACE
    } IMTSLocatorVtbl;

    interface IMTSLocator
    {
        CONST_VTBL struct IMTSLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMTSLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMTSLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMTSLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMTSLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMTSLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMTSLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMTSLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMTSLocator_GetEventDispatcher(This,pUnk)	\
    ( (This)->lpVtbl -> GetEventDispatcher(This,pUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMTSLocator_INTERFACE_DEFINED__ */


#ifndef __IMtsGrp_INTERFACE_DEFINED__
#define __IMtsGrp_INTERFACE_DEFINED__

/* interface IMtsGrp */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMtsGrp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B2E958C-0393-11D1-B1AB-00AA00BA3258")
    IMtsGrp : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long lIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDispatcher) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMtsGrpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMtsGrp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMtsGrp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMtsGrp * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMtsGrp * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMtsGrp * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMtsGrp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMtsGrp * This,
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
        
        DECLSPEC_XFGVIRT(IMtsGrp, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IMtsGrp * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IMtsGrp, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IMtsGrp * This,
            /* [in] */ long lIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDispatcher);
        
        DECLSPEC_XFGVIRT(IMtsGrp, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IMtsGrp * This);
        
        END_INTERFACE
    } IMtsGrpVtbl;

    interface IMtsGrp
    {
        CONST_VTBL struct IMtsGrpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMtsGrp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMtsGrp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMtsGrp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMtsGrp_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMtsGrp_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMtsGrp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMtsGrp_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMtsGrp_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IMtsGrp_Item(This,lIndex,ppUnkDispatcher)	\
    ( (This)->lpVtbl -> Item(This,lIndex,ppUnkDispatcher) ) 

#define IMtsGrp_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMtsGrp_INTERFACE_DEFINED__ */


#ifndef __IMessageMover_INTERFACE_DEFINED__
#define __IMessageMover_INTERFACE_DEFINED__

/* interface IMessageMover */
/* [unique][dual][nonextensible][oleautomation][hidden][object][helpstring][uuid] */ 


EXTERN_C const IID IID_IMessageMover;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("588A085A-B795-11D1-8054-00C04FC340EE")
    IMessageMover : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SourcePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SourcePath( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DestPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DestPath( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CommitBatchSize( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_CommitBatchSize( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveMessages( 
            /* [retval][out] */ __RPC__out long *plMessagesMoved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessageMoverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMessageMover * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMessageMover * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMessageMover * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMessageMover * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMessageMover * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMessageMover * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMessageMover * This,
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
        
        DECLSPEC_XFGVIRT(IMessageMover, get_SourcePath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourcePath )( 
            __RPC__in IMessageMover * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, put_SourcePath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourcePath )( 
            __RPC__in IMessageMover * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, get_DestPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestPath )( 
            __RPC__in IMessageMover * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, put_DestPath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestPath )( 
            __RPC__in IMessageMover * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, get_CommitBatchSize)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommitBatchSize )( 
            __RPC__in IMessageMover * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, put_CommitBatchSize)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CommitBatchSize )( 
            __RPC__in IMessageMover * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IMessageMover, MoveMessages)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveMessages )( 
            __RPC__in IMessageMover * This,
            /* [retval][out] */ __RPC__out long *plMessagesMoved);
        
        END_INTERFACE
    } IMessageMoverVtbl;

    interface IMessageMover
    {
        CONST_VTBL struct IMessageMoverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessageMover_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessageMover_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessageMover_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessageMover_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMessageMover_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMessageMover_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMessageMover_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMessageMover_get_SourcePath(This,pVal)	\
    ( (This)->lpVtbl -> get_SourcePath(This,pVal) ) 

#define IMessageMover_put_SourcePath(This,newVal)	\
    ( (This)->lpVtbl -> put_SourcePath(This,newVal) ) 

#define IMessageMover_get_DestPath(This,pVal)	\
    ( (This)->lpVtbl -> get_DestPath(This,pVal) ) 

#define IMessageMover_put_DestPath(This,newVal)	\
    ( (This)->lpVtbl -> put_DestPath(This,newVal) ) 

#define IMessageMover_get_CommitBatchSize(This,pVal)	\
    ( (This)->lpVtbl -> get_CommitBatchSize(This,pVal) ) 

#define IMessageMover_put_CommitBatchSize(This,newVal)	\
    ( (This)->lpVtbl -> put_CommitBatchSize(This,newVal) ) 

#define IMessageMover_MoveMessages(This,plMessagesMoved)	\
    ( (This)->lpVtbl -> MoveMessages(This,plMessagesMoved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessageMover_INTERFACE_DEFINED__ */


#ifndef __IEventServerTrace_INTERFACE_DEFINED__
#define __IEventServerTrace_INTERFACE_DEFINED__

/* interface IEventServerTrace */
/* [object][unique][helpstring][dual][uuid] */ 


EXTERN_C const IID IID_IEventServerTrace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9A9F12B8-80AF-47ab-A579-35EA57725370")
    IEventServerTrace : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartTraceGuid( 
            /* [in] */ __RPC__in BSTR bstrguidEvent,
            /* [in] */ __RPC__in BSTR bstrguidFilter,
            /* [in] */ LONG lPidFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopTraceGuid( 
            /* [in] */ __RPC__in BSTR bstrguidEvent,
            /* [in] */ __RPC__in BSTR bstrguidFilter,
            /* [in] */ LONG lPidFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumTraceGuid( 
            /* [out] */ __RPC__out LONG *plCntGuids,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrGuidList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventServerTraceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventServerTrace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventServerTrace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventServerTrace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventServerTrace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventServerTrace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventServerTrace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventServerTrace * This,
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
        
        DECLSPEC_XFGVIRT(IEventServerTrace, StartTraceGuid)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartTraceGuid )( 
            __RPC__in IEventServerTrace * This,
            /* [in] */ __RPC__in BSTR bstrguidEvent,
            /* [in] */ __RPC__in BSTR bstrguidFilter,
            /* [in] */ LONG lPidFilter);
        
        DECLSPEC_XFGVIRT(IEventServerTrace, StopTraceGuid)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopTraceGuid )( 
            __RPC__in IEventServerTrace * This,
            /* [in] */ __RPC__in BSTR bstrguidEvent,
            /* [in] */ __RPC__in BSTR bstrguidFilter,
            /* [in] */ LONG lPidFilter);
        
        DECLSPEC_XFGVIRT(IEventServerTrace, EnumTraceGuid)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumTraceGuid )( 
            __RPC__in IEventServerTrace * This,
            /* [out] */ __RPC__out LONG *plCntGuids,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrGuidList);
        
        END_INTERFACE
    } IEventServerTraceVtbl;

    interface IEventServerTrace
    {
        CONST_VTBL struct IEventServerTraceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventServerTrace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventServerTrace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventServerTrace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventServerTrace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventServerTrace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventServerTrace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventServerTrace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventServerTrace_StartTraceGuid(This,bstrguidEvent,bstrguidFilter,lPidFilter)	\
    ( (This)->lpVtbl -> StartTraceGuid(This,bstrguidEvent,bstrguidFilter,lPidFilter) ) 

#define IEventServerTrace_StopTraceGuid(This,bstrguidEvent,bstrguidFilter,lPidFilter)	\
    ( (This)->lpVtbl -> StopTraceGuid(This,bstrguidEvent,bstrguidFilter,lPidFilter) ) 

#define IEventServerTrace_EnumTraceGuid(This,plCntGuids,pbstrGuidList)	\
    ( (This)->lpVtbl -> EnumTraceGuid(This,plCntGuids,pbstrGuidList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventServerTrace_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0049 */
/* [local] */ 

typedef /* [hidden] */ struct _RECYCLE_INFO
    {
    GUID guidCombaseProcessIdentifier;
    LONGLONG ProcessStartTime;
    DWORD dwRecycleLifetimeLimit;
    DWORD dwRecycleMemoryLimit;
    DWORD dwRecycleExpirationTimeout;
    } 	RECYCLE_INFO;

typedef /* [hidden] */ 
enum tagDUMPTYPE
    {
        DUMPTYPE_FULL	= 0,
        DUMPTYPE_MINI	= ( DUMPTYPE_FULL + 1 ) ,
        DUMPTYPE_NONE	= ( DUMPTYPE_MINI + 1 ) 
    } 	DUMPTYPE;

typedef /* [hidden] */ struct _HANG_INFO
    {
    BOOL fAppHangMonitorEnabled;
    BOOL fTerminateOnHang;
    DUMPTYPE DumpType;
    DWORD dwHangTimeout;
    DWORD dwDumpCount;
    DWORD dwInfoMsgCount;
    } 	HANG_INFO;

typedef 
enum tagCOMPLUS_APPTYPE
    {
        APPTYPE_UNKNOWN	= 0xffffffff,
        APPTYPE_SERVER	= 1,
        APPTYPE_LIBRARY	= 0,
        APPTYPE_SWC	= 2
    } 	COMPLUS_APPTYPE;



//
// Definition of global named event used to control starting and 
// stopping of tracker push data.
//
#define TRACKER_STARTSTOP_EVENT L"Global\\COM+ Tracker Push Event"


//
// Definition of global named event used to signal when the 
// system application has been restarted
//
#define TRACKER_INIT_EVENT L"Global\\COM+ Tracker Init Event"


#ifndef GUID_STRING_SIZE
#define GUID_STRING_SIZE				40	    // a couple over.
#endif
typedef /* [hidden] */ struct CAppStatistics
    {
    DWORD m_cTotalCalls;
    DWORD m_cTotalInstances;
    DWORD m_cTotalClasses;
    DWORD m_cCallsPerSecond;
    } 	APPSTATISTICS;

typedef /* [hidden] */ struct CAppData
    {
    DWORD m_idApp;
    WCHAR m_szAppGuid[ 40 ];
    DWORD m_dwAppProcessId;
    APPSTATISTICS m_AppStatistics;
    } 	APPDATA;

typedef /* [hidden] */ struct CCLSIDData
    {
    CLSID m_clsid;
    DWORD m_cReferences;
    DWORD m_cBound;
    DWORD m_cPooled;
    DWORD m_cInCall;
    DWORD m_dwRespTime;
    DWORD m_cCallsCompleted;
    DWORD m_cCallsFailed;
    } 	CLSIDDATA;

typedef /* [hidden] */ struct CCLSIDData2
    {
    CLSID m_clsid;
    GUID m_appid;
    GUID m_partid;
    /* [string] */ WCHAR *m_pwszAppName;
    /* [string] */ WCHAR *m_pwszCtxName;
    COMPLUS_APPTYPE m_eAppType;
    DWORD m_cReferences;
    DWORD m_cBound;
    DWORD m_cPooled;
    DWORD m_cInCall;
    DWORD m_dwRespTime;
    DWORD m_cCallsCompleted;
    DWORD m_cCallsFailed;
    } 	CLSIDDATA2;



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0049_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0049_v0_0_s_ifspec;

#ifndef __IGetAppTrackerData_INTERFACE_DEFINED__
#define __IGetAppTrackerData_INTERFACE_DEFINED__

/* interface IGetAppTrackerData */
/* [helpstring][unique][uuid][object] */ 

#define	DATA_NOT_AVAILABLE	( 0xffffffff )

typedef /* [helpstring] */ 
enum _GetAppTrackerDataFlags
    {
        GATD_INCLUDE_PROCESS_EXE_NAME	= 0x1,
        GATD_INCLUDE_LIBRARY_APPS	= 0x2,
        GATD_INCLUDE_SWC	= 0x4,
        GATD_INCLUDE_CLASS_NAME	= 0x8,
        GATD_INCLUDE_APPLICATION_NAME	= 0x10
    } 	GetAppTrackerDataFlags;

typedef /* [helpstring] */ struct _ApplicationProcessSummary
    {
    GUID PartitionIdPrimaryApplication;
    GUID ApplicationIdPrimaryApplication;
    GUID ApplicationInstanceId;
    DWORD ProcessId;
    COMPLUS_APPTYPE Type;
    /* [string][unique] */ LPWSTR ProcessExeName;
    BOOL IsService;
    BOOL IsPaused;
    BOOL IsRecycled;
    } 	ApplicationProcessSummary;

typedef /* [helpstring] */ struct _ApplicationProcessStatistics
    {
    ULONG NumCallsOutstanding;
    ULONG NumTrackedComponents;
    ULONG NumComponentInstances;
    ULONG AvgCallsPerSecond;
    ULONG Reserved1;
    ULONG Reserved2;
    ULONG Reserved3;
    ULONG Reserved4;
    } 	ApplicationProcessStatistics;

typedef /* [helpstring] */ struct _ApplicationProcessRecycleInfo
    {
    BOOL IsRecyclable;
    BOOL IsRecycled;
    FILETIME TimeRecycled;
    FILETIME TimeToTerminate;
    long RecycleReasonCode;
    BOOL IsPendingRecycle;
    BOOL HasAutomaticLifetimeRecycling;
    FILETIME TimeForAutomaticRecycling;
    ULONG MemoryLimitInKB;
    ULONG MemoryUsageInKBLastCheck;
    ULONG ActivationLimit;
    ULONG NumActivationsLastReported;
    ULONG CallLimit;
    ULONG NumCallsLastReported;
    } 	ApplicationProcessRecycleInfo;

typedef /* [helpstring] */ struct _ApplicationSummary
    {
    GUID ApplicationInstanceId;
    GUID PartitionId;
    GUID ApplicationId;
    COMPLUS_APPTYPE Type;
    /* [string][unique] */ LPWSTR ApplicationName;
    ULONG NumTrackedComponents;
    ULONG NumComponentInstances;
    } 	ApplicationSummary;

typedef /* [helpstring] */ struct _ComponentSummary
    {
    GUID ApplicationInstanceId;
    GUID PartitionId;
    GUID ApplicationId;
    CLSID Clsid;
    /* [string][unique] */ LPWSTR ClassName;
    /* [string][unique] */ LPWSTR ApplicationName;
    } 	ComponentSummary;

typedef /* [helpstring] */ struct _ComponentStatistics
    {
    ULONG NumInstances;
    ULONG NumBoundReferences;
    ULONG NumPooledObjects;
    ULONG NumObjectsInCall;
    ULONG AvgResponseTimeInMs;
    ULONG NumCallsCompletedRecent;
    ULONG NumCallsFailedRecent;
    ULONG NumCallsCompletedTotal;
    ULONG NumCallsFailedTotal;
    ULONG Reserved1;
    ULONG Reserved2;
    ULONG Reserved3;
    ULONG Reserved4;
    } 	ComponentStatistics;

typedef /* [helpstring] */ struct _ComponentHangMonitorInfo
    {
    BOOL IsMonitored;
    BOOL TerminateOnHang;
    ULONG AvgCallThresholdInMs;
    } 	ComponentHangMonitorInfo;


EXTERN_C const IID IID_IGetAppTrackerData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("507C3AC8-3E12-4cb0-9366-653D3E050638")
    IGetAppTrackerData : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetApplicationProcesses( 
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ __RPC__in REFGUID ApplicationId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumApplicationProcesses,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumApplicationProcesses) ApplicationProcessSummary **ApplicationProcesses) = 0;
        
        virtual /* [helpstring][local] */ HRESULT STDMETHODCALLTYPE GetApplicationProcessDetails( 
            /* [in] */ REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ DWORD Flags,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessSummary *Summary,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessStatistics *Statistics,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessRecycleInfo *RecycleInfo,
            /* [annotation] */ 
            _Out_opt_  BOOL *AnyComponentsHangMonitored) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetApplicationsInProcess( 
            /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumApplicationsInProcess,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumApplicationsInProcess) ApplicationSummary **Applications) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetComponentsInProcess( 
            /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ __RPC__in REFGUID ApplicationId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumComponentsInProcess,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumComponentsInProcess) ComponentSummary **Components) = 0;
        
        virtual /* [helpstring][local] */ HRESULT STDMETHODCALLTYPE GetComponentDetails( 
            /* [in] */ REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ REFCLSID Clsid,
            /* [in] */ DWORD Flags,
            /* [annotation] */ 
            _Out_opt_  ComponentSummary *Summary,
            /* [annotation] */ 
            _Out_opt_  ComponentStatistics *Statistics,
            /* [annotation] */ 
            _Out_opt_  ComponentHangMonitorInfo *HangMonitorInfo) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTrackerDataAsCollectionObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **TopLevelCollection) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSuggestedPollingInterval( 
            /* [out] */ __RPC__out DWORD *PollingIntervalInSeconds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetAppTrackerDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetAppTrackerData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetAppTrackerData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetAppTrackerData * This);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetApplicationProcesses)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationProcesses )( 
            __RPC__in IGetAppTrackerData * This,
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ __RPC__in REFGUID ApplicationId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumApplicationProcesses,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumApplicationProcesses) ApplicationProcessSummary **ApplicationProcesses);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetApplicationProcessDetails)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationProcessDetails )( 
            IGetAppTrackerData * This,
            /* [in] */ REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ DWORD Flags,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessSummary *Summary,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessStatistics *Statistics,
            /* [annotation] */ 
            _Out_opt_  ApplicationProcessRecycleInfo *RecycleInfo,
            /* [annotation] */ 
            _Out_opt_  BOOL *AnyComponentsHangMonitored);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetApplicationsInProcess)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationsInProcess )( 
            __RPC__in IGetAppTrackerData * This,
            /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumApplicationsInProcess,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumApplicationsInProcess) ApplicationSummary **Applications);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetComponentsInProcess)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetComponentsInProcess )( 
            __RPC__in IGetAppTrackerData * This,
            /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ __RPC__in REFGUID PartitionId,
            /* [in] */ __RPC__in REFGUID ApplicationId,
            /* [in] */ DWORD Flags,
            /* [out] */ __RPC__out ULONG *NumComponentsInProcess,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*NumComponentsInProcess) ComponentSummary **Components);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetComponentDetails)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *GetComponentDetails )( 
            IGetAppTrackerData * This,
            /* [in] */ REFGUID ApplicationInstanceId,
            /* [in] */ DWORD ProcessId,
            /* [in] */ REFCLSID Clsid,
            /* [in] */ DWORD Flags,
            /* [annotation] */ 
            _Out_opt_  ComponentSummary *Summary,
            /* [annotation] */ 
            _Out_opt_  ComponentStatistics *Statistics,
            /* [annotation] */ 
            _Out_opt_  ComponentHangMonitorInfo *HangMonitorInfo);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetTrackerDataAsCollectionObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTrackerDataAsCollectionObject )( 
            __RPC__in IGetAppTrackerData * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **TopLevelCollection);
        
        DECLSPEC_XFGVIRT(IGetAppTrackerData, GetSuggestedPollingInterval)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSuggestedPollingInterval )( 
            __RPC__in IGetAppTrackerData * This,
            /* [out] */ __RPC__out DWORD *PollingIntervalInSeconds);
        
        END_INTERFACE
    } IGetAppTrackerDataVtbl;

    interface IGetAppTrackerData
    {
        CONST_VTBL struct IGetAppTrackerDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetAppTrackerData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetAppTrackerData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetAppTrackerData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetAppTrackerData_GetApplicationProcesses(This,PartitionId,ApplicationId,Flags,NumApplicationProcesses,ApplicationProcesses)	\
    ( (This)->lpVtbl -> GetApplicationProcesses(This,PartitionId,ApplicationId,Flags,NumApplicationProcesses,ApplicationProcesses) ) 

#define IGetAppTrackerData_GetApplicationProcessDetails(This,ApplicationInstanceId,ProcessId,Flags,Summary,Statistics,RecycleInfo,AnyComponentsHangMonitored)	\
    ( (This)->lpVtbl -> GetApplicationProcessDetails(This,ApplicationInstanceId,ProcessId,Flags,Summary,Statistics,RecycleInfo,AnyComponentsHangMonitored) ) 

#define IGetAppTrackerData_GetApplicationsInProcess(This,ApplicationInstanceId,ProcessId,PartitionId,Flags,NumApplicationsInProcess,Applications)	\
    ( (This)->lpVtbl -> GetApplicationsInProcess(This,ApplicationInstanceId,ProcessId,PartitionId,Flags,NumApplicationsInProcess,Applications) ) 

#define IGetAppTrackerData_GetComponentsInProcess(This,ApplicationInstanceId,ProcessId,PartitionId,ApplicationId,Flags,NumComponentsInProcess,Components)	\
    ( (This)->lpVtbl -> GetComponentsInProcess(This,ApplicationInstanceId,ProcessId,PartitionId,ApplicationId,Flags,NumComponentsInProcess,Components) ) 

#define IGetAppTrackerData_GetComponentDetails(This,ApplicationInstanceId,ProcessId,Clsid,Flags,Summary,Statistics,HangMonitorInfo)	\
    ( (This)->lpVtbl -> GetComponentDetails(This,ApplicationInstanceId,ProcessId,Clsid,Flags,Summary,Statistics,HangMonitorInfo) ) 

#define IGetAppTrackerData_GetTrackerDataAsCollectionObject(This,TopLevelCollection)	\
    ( (This)->lpVtbl -> GetTrackerDataAsCollectionObject(This,TopLevelCollection) ) 

#define IGetAppTrackerData_GetSuggestedPollingInterval(This,PollingIntervalInSeconds)	\
    ( (This)->lpVtbl -> GetSuggestedPollingInterval(This,PollingIntervalInSeconds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_RemoteGetApplicationProcessDetails_Proxy( 
    __RPC__in IGetAppTrackerData * This,
    /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ DWORD Flags,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessSummary *Summary,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessStatistics *Statistics,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessRecycleInfo *RecycleInfo,
    /* [unique][out][in] */ __RPC__inout_opt BOOL *AnyComponentsHangMonitored);


void __RPC_STUB IGetAppTrackerData_RemoteGetApplicationProcessDetails_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_RemoteGetComponentDetails_Proxy( 
    __RPC__in IGetAppTrackerData * This,
    /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ __RPC__in REFCLSID Clsid,
    /* [in] */ DWORD Flags,
    /* [unique][out][in] */ __RPC__inout_opt ComponentSummary *Summary,
    /* [unique][out][in] */ __RPC__inout_opt ComponentStatistics *Statistics,
    /* [unique][out][in] */ __RPC__inout_opt ComponentHangMonitorInfo *HangMonitorInfo);


void __RPC_STUB IGetAppTrackerData_RemoteGetComponentDetails_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IGetAppTrackerData_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0050 */
/* [local] */ 

//
// Dispenser Manager interfaces
//
//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.
 
#ifndef DECLSPEC_UUID
#if (_MSC_VER >= 1100) && defined (__cplusplus)
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif
#endif
typedef DWORD_PTR RESTYPID;

typedef DWORD_PTR RESID;

typedef LPOLESTR SRESID;

typedef LPCOLESTR constSRESID;

typedef DWORD RESOURCERATING;

typedef long TIMEINSECS;

typedef DWORD_PTR INSTID;

typedef DWORD_PTR TRANSID;



//
// Error Codes
//
#define MTXDM_E_ENLISTRESOURCEFAILED 0x8004E100 // return from EnlistResource, is then returned by AllocResource
 
//
// IDispenserManager
// Implemented by Dispenser Manager, called by all Dispensers.
//


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0050_v0_0_s_ifspec;

#ifndef __IDispenserManager_INTERFACE_DEFINED__
#define __IDispenserManager_INTERFACE_DEFINED__

/* interface IDispenserManager */
/* [unique][hidden][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDispenserManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5cb31e10-2b5f-11cf-be10-00aa00a2fa25")
    IDispenserManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterDispenser( 
            /* [in] */ IDispenserDriver *__MIDL__IDispenserManager0000,
            /* [in] */ LPCOLESTR szDispenserName,
            /* [out] */ IHolder **__MIDL__IDispenserManager0001) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [out] */ INSTID *__MIDL__IDispenserManager0002,
            /* [out] */ TRANSID *__MIDL__IDispenserManager0003) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispenserManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDispenserManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDispenserManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDispenserManager * This);
        
        DECLSPEC_XFGVIRT(IDispenserManager, RegisterDispenser)
        HRESULT ( STDMETHODCALLTYPE *RegisterDispenser )( 
            IDispenserManager * This,
            /* [in] */ IDispenserDriver *__MIDL__IDispenserManager0000,
            /* [in] */ LPCOLESTR szDispenserName,
            /* [out] */ IHolder **__MIDL__IDispenserManager0001);
        
        DECLSPEC_XFGVIRT(IDispenserManager, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            IDispenserManager * This,
            /* [out] */ INSTID *__MIDL__IDispenserManager0002,
            /* [out] */ TRANSID *__MIDL__IDispenserManager0003);
        
        END_INTERFACE
    } IDispenserManagerVtbl;

    interface IDispenserManager
    {
        CONST_VTBL struct IDispenserManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispenserManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispenserManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispenserManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispenserManager_RegisterDispenser(This,__MIDL__IDispenserManager0000,szDispenserName,__MIDL__IDispenserManager0001)	\
    ( (This)->lpVtbl -> RegisterDispenser(This,__MIDL__IDispenserManager0000,szDispenserName,__MIDL__IDispenserManager0001) ) 

#define IDispenserManager_GetContext(This,__MIDL__IDispenserManager0002,__MIDL__IDispenserManager0003)	\
    ( (This)->lpVtbl -> GetContext(This,__MIDL__IDispenserManager0002,__MIDL__IDispenserManager0003) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDispenserManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0051 */
/* [local] */ 

//
// IHolder
// Implemented by Dispenser Manager, called by one Dispenser.
//


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0051_v0_0_s_ifspec;

#ifndef __IHolder_INTERFACE_DEFINED__
#define __IHolder_INTERFACE_DEFINED__

/* interface IHolder */
/* [unique][hidden][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IHolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bf6a1850-2b45-11cf-be10-00aa00a2fa25")
    IHolder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AllocResource( 
            /* [in] */ const RESTYPID __MIDL__IHolder0000,
            /* [out] */ RESID *__MIDL__IHolder0001) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeResource( 
            /* [in] */ const RESID __MIDL__IHolder0002) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TrackResource( 
            /* [in] */ const RESID __MIDL__IHolder0003) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TrackResourceS( 
            /* [in] */ constSRESID __MIDL__IHolder0004) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UntrackResource( 
            /* [in] */ const RESID __MIDL__IHolder0005,
            /* [in] */ const BOOL __MIDL__IHolder0006) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UntrackResourceS( 
            /* [in] */ constSRESID __MIDL__IHolder0007,
            /* [in] */ const BOOL __MIDL__IHolder0008) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestDestroyResource( 
            /* [in] */ const RESID __MIDL__IHolder0009) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHolder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHolder * This);
        
        DECLSPEC_XFGVIRT(IHolder, AllocResource)
        HRESULT ( STDMETHODCALLTYPE *AllocResource )( 
            IHolder * This,
            /* [in] */ const RESTYPID __MIDL__IHolder0000,
            /* [out] */ RESID *__MIDL__IHolder0001);
        
        DECLSPEC_XFGVIRT(IHolder, FreeResource)
        HRESULT ( STDMETHODCALLTYPE *FreeResource )( 
            IHolder * This,
            /* [in] */ const RESID __MIDL__IHolder0002);
        
        DECLSPEC_XFGVIRT(IHolder, TrackResource)
        HRESULT ( STDMETHODCALLTYPE *TrackResource )( 
            IHolder * This,
            /* [in] */ const RESID __MIDL__IHolder0003);
        
        DECLSPEC_XFGVIRT(IHolder, TrackResourceS)
        HRESULT ( STDMETHODCALLTYPE *TrackResourceS )( 
            IHolder * This,
            /* [in] */ constSRESID __MIDL__IHolder0004);
        
        DECLSPEC_XFGVIRT(IHolder, UntrackResource)
        HRESULT ( STDMETHODCALLTYPE *UntrackResource )( 
            IHolder * This,
            /* [in] */ const RESID __MIDL__IHolder0005,
            /* [in] */ const BOOL __MIDL__IHolder0006);
        
        DECLSPEC_XFGVIRT(IHolder, UntrackResourceS)
        HRESULT ( STDMETHODCALLTYPE *UntrackResourceS )( 
            IHolder * This,
            /* [in] */ constSRESID __MIDL__IHolder0007,
            /* [in] */ const BOOL __MIDL__IHolder0008);
        
        DECLSPEC_XFGVIRT(IHolder, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IHolder * This);
        
        DECLSPEC_XFGVIRT(IHolder, RequestDestroyResource)
        HRESULT ( STDMETHODCALLTYPE *RequestDestroyResource )( 
            IHolder * This,
            /* [in] */ const RESID __MIDL__IHolder0009);
        
        END_INTERFACE
    } IHolderVtbl;

    interface IHolder
    {
        CONST_VTBL struct IHolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHolder_AllocResource(This,__MIDL__IHolder0000,__MIDL__IHolder0001)	\
    ( (This)->lpVtbl -> AllocResource(This,__MIDL__IHolder0000,__MIDL__IHolder0001) ) 

#define IHolder_FreeResource(This,__MIDL__IHolder0002)	\
    ( (This)->lpVtbl -> FreeResource(This,__MIDL__IHolder0002) ) 

#define IHolder_TrackResource(This,__MIDL__IHolder0003)	\
    ( (This)->lpVtbl -> TrackResource(This,__MIDL__IHolder0003) ) 

#define IHolder_TrackResourceS(This,__MIDL__IHolder0004)	\
    ( (This)->lpVtbl -> TrackResourceS(This,__MIDL__IHolder0004) ) 

#define IHolder_UntrackResource(This,__MIDL__IHolder0005,__MIDL__IHolder0006)	\
    ( (This)->lpVtbl -> UntrackResource(This,__MIDL__IHolder0005,__MIDL__IHolder0006) ) 

#define IHolder_UntrackResourceS(This,__MIDL__IHolder0007,__MIDL__IHolder0008)	\
    ( (This)->lpVtbl -> UntrackResourceS(This,__MIDL__IHolder0007,__MIDL__IHolder0008) ) 

#define IHolder_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IHolder_RequestDestroyResource(This,__MIDL__IHolder0009)	\
    ( (This)->lpVtbl -> RequestDestroyResource(This,__MIDL__IHolder0009) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHolder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0052 */
/* [local] */ 

//
// IDispenserDriver
// Implemented by a Dispenser, called by Dispenser Manager.
//


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0052_v0_0_s_ifspec;

#ifndef __IDispenserDriver_INTERFACE_DEFINED__
#define __IDispenserDriver_INTERFACE_DEFINED__

/* interface IDispenserDriver */
/* [unique][hidden][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDispenserDriver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("208b3651-2b48-11cf-be10-00aa00a2fa25")
    IDispenserDriver : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateResource( 
            /* [in] */ const RESTYPID ResTypId,
            /* [out] */ RESID *pResId,
            /* [out] */ TIMEINSECS *pSecsFreeBeforeDestroy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RateResource( 
            /* [in] */ const RESTYPID ResTypId,
            /* [in] */ const RESID ResId,
            /* [in] */ const BOOL fRequiresTransactionEnlistment,
            /* [out] */ RESOURCERATING *pRating) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnlistResource( 
            /* [in] */ const RESID ResId,
            /* [in] */ const TRANSID TransId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetResource( 
            /* [in] */ const RESID ResId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyResource( 
            /* [in] */ const RESID ResId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyResourceS( 
            /* [in] */ constSRESID ResId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispenserDriverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDispenserDriver * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDispenserDriver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDispenserDriver * This);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, CreateResource)
        HRESULT ( STDMETHODCALLTYPE *CreateResource )( 
            IDispenserDriver * This,
            /* [in] */ const RESTYPID ResTypId,
            /* [out] */ RESID *pResId,
            /* [out] */ TIMEINSECS *pSecsFreeBeforeDestroy);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, RateResource)
        HRESULT ( STDMETHODCALLTYPE *RateResource )( 
            IDispenserDriver * This,
            /* [in] */ const RESTYPID ResTypId,
            /* [in] */ const RESID ResId,
            /* [in] */ const BOOL fRequiresTransactionEnlistment,
            /* [out] */ RESOURCERATING *pRating);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, EnlistResource)
        HRESULT ( STDMETHODCALLTYPE *EnlistResource )( 
            IDispenserDriver * This,
            /* [in] */ const RESID ResId,
            /* [in] */ const TRANSID TransId);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, ResetResource)
        HRESULT ( STDMETHODCALLTYPE *ResetResource )( 
            IDispenserDriver * This,
            /* [in] */ const RESID ResId);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, DestroyResource)
        HRESULT ( STDMETHODCALLTYPE *DestroyResource )( 
            IDispenserDriver * This,
            /* [in] */ const RESID ResId);
        
        DECLSPEC_XFGVIRT(IDispenserDriver, DestroyResourceS)
        HRESULT ( STDMETHODCALLTYPE *DestroyResourceS )( 
            IDispenserDriver * This,
            /* [in] */ constSRESID ResId);
        
        END_INTERFACE
    } IDispenserDriverVtbl;

    interface IDispenserDriver
    {
        CONST_VTBL struct IDispenserDriverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispenserDriver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispenserDriver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispenserDriver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispenserDriver_CreateResource(This,ResTypId,pResId,pSecsFreeBeforeDestroy)	\
    ( (This)->lpVtbl -> CreateResource(This,ResTypId,pResId,pSecsFreeBeforeDestroy) ) 

#define IDispenserDriver_RateResource(This,ResTypId,ResId,fRequiresTransactionEnlistment,pRating)	\
    ( (This)->lpVtbl -> RateResource(This,ResTypId,ResId,fRequiresTransactionEnlistment,pRating) ) 

#define IDispenserDriver_EnlistResource(This,ResId,TransId)	\
    ( (This)->lpVtbl -> EnlistResource(This,ResId,TransId) ) 

#define IDispenserDriver_ResetResource(This,ResId)	\
    ( (This)->lpVtbl -> ResetResource(This,ResId) ) 

#define IDispenserDriver_DestroyResource(This,ResId)	\
    ( (This)->lpVtbl -> DestroyResource(This,ResId) ) 

#define IDispenserDriver_DestroyResourceS(This,ResId)	\
    ( (This)->lpVtbl -> DestroyResourceS(This,ResId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDispenserDriver_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0053 */
/* [local] */ 

#ifdef USE_UUIDOF_FOR_IID_
#define  IID_IHolder             __uuidof(IIHolder)
#define  IID_IDispenserManager   __uuidof(IDispenserManager)
#define  IID_IDispenserDriver    __uuidof(IDispenserDriver)
#endif


////////////////////////////////////////////////////////////
// This is the list of Microsoft-allocated process recycling
// reason codes.   These are typed as a long; all values with the
// high bit set are reserved by Microsoft.    Users who have no
// special information to add may use the CRR_NO_REASON_SUPPLIED
// code for that purpose.   The value zero is reserved for the
// CRR_NO_REASON_SUPPLIED code.


#define CRR_NO_REASON_SUPPLIED  0x00000000
#define CRR_LIFETIME_LIMIT      0xFFFFFFFF
#define CRR_ACTIVATION_LIMIT    0xFFFFFFFE
#define CRR_CALL_LIMIT          0xFFFFFFFD
#define CRR_MEMORY_LIMIT        0xFFFFFFFC
#define CRR_RECYCLED_FROM_UI    0xFFFFFFFB


////////////////////////////////////////////////////////////


EXTERN_C const CLSID CLSID_MTSPackage;
EXTERN_C const GUID  GUID_DefaultAppPartition;
EXTERN_C const GUID  GUID_FinalizerCID;
EXTERN_C const GUID  IID_IEnterActivityWithNoLock;


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0053_v0_0_s_ifspec;

#ifndef __ITransactionProxy_INTERFACE_DEFINED__
#define __ITransactionProxy_INTERFACE_DEFINED__

/* interface ITransactionProxy */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ITransactionProxy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02558374-DF2E-4dae-BD6B-1D5C994F9BDC")
    ITransactionProxy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ GUID guid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Promote( 
            /* [out] */ ITransaction **pTransaction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateVoter( 
            /* [in] */ ITransactionVoterNotifyAsync2 *pTxAsync,
            /* [out] */ ITransactionVoterBallotAsync2 **ppBallot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsolationLevel( 
            /* [retval][out] */ ISOLEVEL *__MIDL__ITransactionProxy0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIdentifier( 
            /* [retval][out] */ GUID *pbstrIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsReusable( 
            /* [out] */ BOOL *pfIsReusable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionProxyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITransactionProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITransactionProxy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITransactionProxy * This);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            ITransactionProxy * This,
            /* [in] */ GUID guid);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            ITransactionProxy * This);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, Promote)
        HRESULT ( STDMETHODCALLTYPE *Promote )( 
            ITransactionProxy * This,
            /* [out] */ ITransaction **pTransaction);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, CreateVoter)
        HRESULT ( STDMETHODCALLTYPE *CreateVoter )( 
            ITransactionProxy * This,
            /* [in] */ ITransactionVoterNotifyAsync2 *pTxAsync,
            /* [out] */ ITransactionVoterBallotAsync2 **ppBallot);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, GetIsolationLevel)
        HRESULT ( STDMETHODCALLTYPE *GetIsolationLevel )( 
            ITransactionProxy * This,
            /* [retval][out] */ ISOLEVEL *__MIDL__ITransactionProxy0000);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, GetIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetIdentifier )( 
            ITransactionProxy * This,
            /* [retval][out] */ GUID *pbstrIdentifier);
        
        DECLSPEC_XFGVIRT(ITransactionProxy, IsReusable)
        HRESULT ( STDMETHODCALLTYPE *IsReusable )( 
            ITransactionProxy * This,
            /* [out] */ BOOL *pfIsReusable);
        
        END_INTERFACE
    } ITransactionProxyVtbl;

    interface ITransactionProxy
    {
        CONST_VTBL struct ITransactionProxyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionProxy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionProxy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionProxy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionProxy_Commit(This,guid)	\
    ( (This)->lpVtbl -> Commit(This,guid) ) 

#define ITransactionProxy_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define ITransactionProxy_Promote(This,pTransaction)	\
    ( (This)->lpVtbl -> Promote(This,pTransaction) ) 

#define ITransactionProxy_CreateVoter(This,pTxAsync,ppBallot)	\
    ( (This)->lpVtbl -> CreateVoter(This,pTxAsync,ppBallot) ) 

#define ITransactionProxy_GetIsolationLevel(This,__MIDL__ITransactionProxy0000)	\
    ( (This)->lpVtbl -> GetIsolationLevel(This,__MIDL__ITransactionProxy0000) ) 

#define ITransactionProxy_GetIdentifier(This,pbstrIdentifier)	\
    ( (This)->lpVtbl -> GetIdentifier(This,pbstrIdentifier) ) 

#define ITransactionProxy_IsReusable(This,pfIsReusable)	\
    ( (This)->lpVtbl -> IsReusable(This,pfIsReusable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionProxy_INTERFACE_DEFINED__ */


#ifndef __IContextSecurityPerimeter_INTERFACE_DEFINED__
#define __IContextSecurityPerimeter_INTERFACE_DEFINED__

/* interface IContextSecurityPerimeter */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IContextSecurityPerimeter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A7549A29-A7C4-42e1-8DC1-7E3D748DC24A")
    IContextSecurityPerimeter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPerimeterFlag( 
            /* [out] */ BOOL *pFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPerimeterFlag( 
            /* [in] */ BOOL fFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextSecurityPerimeterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IContextSecurityPerimeter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IContextSecurityPerimeter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IContextSecurityPerimeter * This);
        
        DECLSPEC_XFGVIRT(IContextSecurityPerimeter, GetPerimeterFlag)
        HRESULT ( STDMETHODCALLTYPE *GetPerimeterFlag )( 
            IContextSecurityPerimeter * This,
            /* [out] */ BOOL *pFlag);
        
        DECLSPEC_XFGVIRT(IContextSecurityPerimeter, SetPerimeterFlag)
        HRESULT ( STDMETHODCALLTYPE *SetPerimeterFlag )( 
            IContextSecurityPerimeter * This,
            /* [in] */ BOOL fFlag);
        
        END_INTERFACE
    } IContextSecurityPerimeterVtbl;

    interface IContextSecurityPerimeter
    {
        CONST_VTBL struct IContextSecurityPerimeterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextSecurityPerimeter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextSecurityPerimeter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextSecurityPerimeter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextSecurityPerimeter_GetPerimeterFlag(This,pFlag)	\
    ( (This)->lpVtbl -> GetPerimeterFlag(This,pFlag) ) 

#define IContextSecurityPerimeter_SetPerimeterFlag(This,fFlag)	\
    ( (This)->lpVtbl -> SetPerimeterFlag(This,fFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextSecurityPerimeter_INTERFACE_DEFINED__ */


#ifndef __ITxProxyHolder_INTERFACE_DEFINED__
#define __ITxProxyHolder_INTERFACE_DEFINED__

/* interface ITxProxyHolder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ITxProxyHolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13D86F31-0139-41af-BCAD-C7D50435FE9F")
    ITxProxyHolder : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE GetIdentifier( 
            /* [out] */ GUID *pGuidLtx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITxProxyHolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITxProxyHolder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITxProxyHolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITxProxyHolder * This);
        
        DECLSPEC_XFGVIRT(ITxProxyHolder, GetIdentifier)
        void ( STDMETHODCALLTYPE *GetIdentifier )( 
            ITxProxyHolder * This,
            /* [out] */ GUID *pGuidLtx);
        
        END_INTERFACE
    } ITxProxyHolderVtbl;

    interface ITxProxyHolder
    {
        CONST_VTBL struct ITxProxyHolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITxProxyHolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITxProxyHolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITxProxyHolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITxProxyHolder_GetIdentifier(This,pGuidLtx)	\
    ( (This)->lpVtbl -> GetIdentifier(This,pGuidLtx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITxProxyHolder_INTERFACE_DEFINED__ */


#ifndef __IObjectContext_INTERFACE_DEFINED__
#define __IObjectContext_INTERFACE_DEFINED__

/* interface IObjectContext */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IObjectContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372ae0-cae7-11cf-be81-00aa00a2fa25")
    IObjectContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFIID riid,
            /* [retval][iid_is][out] */ LPVOID *ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAbort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableCommit( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableCommit( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsInTransaction( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsSecurityEnabled( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCallerInRole( 
            /* [in] */ BSTR bstrRole,
            /* [retval][out] */ BOOL *pfIsInRole) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IObjectContext * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFIID riid,
            /* [retval][iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IObjectContext, SetComplete)
        HRESULT ( STDMETHODCALLTYPE *SetComplete )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, SetAbort)
        HRESULT ( STDMETHODCALLTYPE *SetAbort )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, EnableCommit)
        HRESULT ( STDMETHODCALLTYPE *EnableCommit )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, DisableCommit)
        HRESULT ( STDMETHODCALLTYPE *DisableCommit )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, IsInTransaction)
        BOOL ( STDMETHODCALLTYPE *IsInTransaction )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, IsSecurityEnabled)
        BOOL ( STDMETHODCALLTYPE *IsSecurityEnabled )( 
            IObjectContext * This);
        
        DECLSPEC_XFGVIRT(IObjectContext, IsCallerInRole)
        HRESULT ( STDMETHODCALLTYPE *IsCallerInRole )( 
            IObjectContext * This,
            /* [in] */ BSTR bstrRole,
            /* [retval][out] */ BOOL *pfIsInRole);
        
        END_INTERFACE
    } IObjectContextVtbl;

    interface IObjectContext
    {
        CONST_VTBL struct IObjectContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectContext_CreateInstance(This,rclsid,riid,ppv)	\
    ( (This)->lpVtbl -> CreateInstance(This,rclsid,riid,ppv) ) 

#define IObjectContext_SetComplete(This)	\
    ( (This)->lpVtbl -> SetComplete(This) ) 

#define IObjectContext_SetAbort(This)	\
    ( (This)->lpVtbl -> SetAbort(This) ) 

#define IObjectContext_EnableCommit(This)	\
    ( (This)->lpVtbl -> EnableCommit(This) ) 

#define IObjectContext_DisableCommit(This)	\
    ( (This)->lpVtbl -> DisableCommit(This) ) 

#define IObjectContext_IsInTransaction(This)	\
    ( (This)->lpVtbl -> IsInTransaction(This) ) 

#define IObjectContext_IsSecurityEnabled(This)	\
    ( (This)->lpVtbl -> IsSecurityEnabled(This) ) 

#define IObjectContext_IsCallerInRole(This,bstrRole,pfIsInRole)	\
    ( (This)->lpVtbl -> IsCallerInRole(This,bstrRole,pfIsInRole) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectContext_INTERFACE_DEFINED__ */


#ifndef __IObjectControl_INTERFACE_DEFINED__
#define __IObjectControl_INTERFACE_DEFINED__

/* interface IObjectControl */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IObjectControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372aec-cae7-11cf-be81-00aa00a2fa25")
    IObjectControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Activate( void) = 0;
        
        virtual void STDMETHODCALLTYPE Deactivate( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE CanBePooled( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectControl * This);
        
        DECLSPEC_XFGVIRT(IObjectControl, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IObjectControl * This);
        
        DECLSPEC_XFGVIRT(IObjectControl, Deactivate)
        void ( STDMETHODCALLTYPE *Deactivate )( 
            IObjectControl * This);
        
        DECLSPEC_XFGVIRT(IObjectControl, CanBePooled)
        BOOL ( STDMETHODCALLTYPE *CanBePooled )( 
            IObjectControl * This);
        
        END_INTERFACE
    } IObjectControlVtbl;

    interface IObjectControl
    {
        CONST_VTBL struct IObjectControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectControl_Activate(This)	\
    ( (This)->lpVtbl -> Activate(This) ) 

#define IObjectControl_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define IObjectControl_CanBePooled(This)	\
    ( (This)->lpVtbl -> CanBePooled(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectControl_INTERFACE_DEFINED__ */


#ifndef __IEnumNames_INTERFACE_DEFINED__
#define __IEnumNames_INTERFACE_DEFINED__

/* interface IEnumNames */
/* [object][unique][uuid][hidden][local] */ 


EXTERN_C const IID IID_IEnumNames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372af2-cae7-11cf-be81-00aa00a2fa25")
    IEnumNames : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ unsigned long celt,
            /* [size_is][out] */ BSTR *rgname,
            /* [retval][out] */ unsigned long *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ unsigned long celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumNames **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumNamesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumNames * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumNames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumNames * This);
        
        DECLSPEC_XFGVIRT(IEnumNames, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumNames * This,
            /* [in] */ unsigned long celt,
            /* [size_is][out] */ BSTR *rgname,
            /* [retval][out] */ unsigned long *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumNames, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumNames * This,
            /* [in] */ unsigned long celt);
        
        DECLSPEC_XFGVIRT(IEnumNames, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumNames * This);
        
        DECLSPEC_XFGVIRT(IEnumNames, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumNames * This,
            /* [retval][out] */ IEnumNames **ppenum);
        
        END_INTERFACE
    } IEnumNamesVtbl;

    interface IEnumNames
    {
        CONST_VTBL struct IEnumNamesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumNames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumNames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumNames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumNames_Next(This,celt,rgname,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgname,pceltFetched) ) 

#define IEnumNames_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumNames_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumNames_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumNames_INTERFACE_DEFINED__ */


#ifndef __ISecurityProperty_INTERFACE_DEFINED__
#define __ISecurityProperty_INTERFACE_DEFINED__

/* interface ISecurityProperty */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_ISecurityProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372aea-cae7-11cf-be81-00aa00a2fa25")
    ISecurityProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDirectCreatorSID( 
            PSID *pSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalCreatorSID( 
            PSID *pSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDirectCallerSID( 
            PSID *pSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalCallerSID( 
            PSID *pSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseSID( 
            PSID pSID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISecurityPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISecurityProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISecurityProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISecurityProperty * This);
        
        DECLSPEC_XFGVIRT(ISecurityProperty, GetDirectCreatorSID)
        HRESULT ( STDMETHODCALLTYPE *GetDirectCreatorSID )( 
            ISecurityProperty * This,
            PSID *pSID);
        
        DECLSPEC_XFGVIRT(ISecurityProperty, GetOriginalCreatorSID)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalCreatorSID )( 
            ISecurityProperty * This,
            PSID *pSID);
        
        DECLSPEC_XFGVIRT(ISecurityProperty, GetDirectCallerSID)
        HRESULT ( STDMETHODCALLTYPE *GetDirectCallerSID )( 
            ISecurityProperty * This,
            PSID *pSID);
        
        DECLSPEC_XFGVIRT(ISecurityProperty, GetOriginalCallerSID)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalCallerSID )( 
            ISecurityProperty * This,
            PSID *pSID);
        
        DECLSPEC_XFGVIRT(ISecurityProperty, ReleaseSID)
        HRESULT ( STDMETHODCALLTYPE *ReleaseSID )( 
            ISecurityProperty * This,
            PSID pSID);
        
        END_INTERFACE
    } ISecurityPropertyVtbl;

    interface ISecurityProperty
    {
        CONST_VTBL struct ISecurityPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISecurityProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISecurityProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISecurityProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISecurityProperty_GetDirectCreatorSID(This,pSID)	\
    ( (This)->lpVtbl -> GetDirectCreatorSID(This,pSID) ) 

#define ISecurityProperty_GetOriginalCreatorSID(This,pSID)	\
    ( (This)->lpVtbl -> GetOriginalCreatorSID(This,pSID) ) 

#define ISecurityProperty_GetDirectCallerSID(This,pSID)	\
    ( (This)->lpVtbl -> GetDirectCallerSID(This,pSID) ) 

#define ISecurityProperty_GetOriginalCallerSID(This,pSID)	\
    ( (This)->lpVtbl -> GetOriginalCallerSID(This,pSID) ) 

#define ISecurityProperty_ReleaseSID(This,pSID)	\
    ( (This)->lpVtbl -> ReleaseSID(This,pSID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISecurityProperty_INTERFACE_DEFINED__ */


#ifndef __ObjectControl_INTERFACE_DEFINED__
#define __ObjectControl_INTERFACE_DEFINED__

/* interface ObjectControl */
/* [version][helpcontext][helpstring][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ObjectControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7DC41850-0C31-11d0-8B79-00AA00B8A790")
    ObjectControl : public IUnknown
    {
    public:
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE Activate( void) = 0;
        
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE Deactivate( void) = 0;
        
        virtual /* [helpstring][helpcontext] */ HRESULT STDMETHODCALLTYPE CanBePooled( 
            /* [retval][out] */ VARIANT_BOOL *pbPoolable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ObjectControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ObjectControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ObjectControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ObjectControl * This);
        
        DECLSPEC_XFGVIRT(ObjectControl, Activate)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Activate )( 
            ObjectControl * This);
        
        DECLSPEC_XFGVIRT(ObjectControl, Deactivate)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            ObjectControl * This);
        
        DECLSPEC_XFGVIRT(ObjectControl, CanBePooled)
        /* [helpstring][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CanBePooled )( 
            ObjectControl * This,
            /* [retval][out] */ VARIANT_BOOL *pbPoolable);
        
        END_INTERFACE
    } ObjectControlVtbl;

    interface ObjectControl
    {
        CONST_VTBL struct ObjectControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ObjectControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ObjectControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ObjectControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ObjectControl_Activate(This)	\
    ( (This)->lpVtbl -> Activate(This) ) 

#define ObjectControl_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define ObjectControl_CanBePooled(This,pbPoolable)	\
    ( (This)->lpVtbl -> CanBePooled(This,pbPoolable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ObjectControl_INTERFACE_DEFINED__ */


#ifndef __ISharedProperty_INTERFACE_DEFINED__
#define __ISharedProperty_INTERFACE_DEFINED__

/* interface ISharedProperty */
/* [object][unique][helpcontext][helpstring][dual][uuid] */ 


EXTERN_C const IID IID_ISharedProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A005C01-A5DE-11CF-9E66-00AA00A3F464")
    ISharedProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT val) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISharedPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISharedProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISharedProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISharedProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISharedProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISharedProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISharedProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISharedProperty * This,
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
        
        DECLSPEC_XFGVIRT(ISharedProperty, get_Value)
        /* [helpstring][helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ISharedProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ISharedProperty, put_Value)
        /* [helpstring][helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ISharedProperty * This,
            /* [in] */ VARIANT val);
        
        END_INTERFACE
    } ISharedPropertyVtbl;

    interface ISharedProperty
    {
        CONST_VTBL struct ISharedPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISharedProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISharedProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISharedProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISharedProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISharedProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISharedProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISharedProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISharedProperty_get_Value(This,pVal)	\
    ( (This)->lpVtbl -> get_Value(This,pVal) ) 

#define ISharedProperty_put_Value(This,val)	\
    ( (This)->lpVtbl -> put_Value(This,val) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISharedProperty_INTERFACE_DEFINED__ */


#ifndef __ISharedPropertyGroup_INTERFACE_DEFINED__
#define __ISharedPropertyGroup_INTERFACE_DEFINED__

/* interface ISharedPropertyGroup */
/* [object][unique][helpcontext][helpstring][dual][uuid] */ 


EXTERN_C const IID IID_ISharedPropertyGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A005C07-A5DE-11CF-9E66-00AA00A3F464")
    ISharedPropertyGroup : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreatePropertyByPosition( 
            /* [in] */ int Index,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProp) = 0;
        
        virtual /* [helpstring][helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_PropertyByPosition( 
            /* [in] */ int Index,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProperty) = 0;
        
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreateProperty( 
            /* [in] */ __RPC__in BSTR Name,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProp) = 0;
        
        virtual /* [helpstring][helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Property( 
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProperty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISharedPropertyGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISharedPropertyGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISharedPropertyGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISharedPropertyGroup * This,
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
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroup, CreatePropertyByPosition)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertyByPosition )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ int Index,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProp);
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroup, get_PropertyByPosition)
        /* [helpstring][helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PropertyByPosition )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ int Index,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProperty);
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroup, CreateProperty)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateProperty )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProp);
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroup, get_Property)
        /* [helpstring][helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Property )( 
            __RPC__in ISharedPropertyGroup * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ISharedProperty **ppProperty);
        
        END_INTERFACE
    } ISharedPropertyGroupVtbl;

    interface ISharedPropertyGroup
    {
        CONST_VTBL struct ISharedPropertyGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISharedPropertyGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISharedPropertyGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISharedPropertyGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISharedPropertyGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISharedPropertyGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISharedPropertyGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISharedPropertyGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISharedPropertyGroup_CreatePropertyByPosition(This,Index,fExists,ppProp)	\
    ( (This)->lpVtbl -> CreatePropertyByPosition(This,Index,fExists,ppProp) ) 

#define ISharedPropertyGroup_get_PropertyByPosition(This,Index,ppProperty)	\
    ( (This)->lpVtbl -> get_PropertyByPosition(This,Index,ppProperty) ) 

#define ISharedPropertyGroup_CreateProperty(This,Name,fExists,ppProp)	\
    ( (This)->lpVtbl -> CreateProperty(This,Name,fExists,ppProp) ) 

#define ISharedPropertyGroup_get_Property(This,Name,ppProperty)	\
    ( (This)->lpVtbl -> get_Property(This,Name,ppProperty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISharedPropertyGroup_INTERFACE_DEFINED__ */


#ifndef __ISharedPropertyGroupManager_INTERFACE_DEFINED__
#define __ISharedPropertyGroupManager_INTERFACE_DEFINED__

/* interface ISharedPropertyGroupManager */
/* [object][unique][helpcontext][helpstring][dual][uuid] */ 


EXTERN_C const IID IID_ISharedPropertyGroupManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A005C0D-A5DE-11CF-9E66-00AA00A3F464")
    ISharedPropertyGroupManager : public IDispatch
    {
    public:
        virtual /* [helpstring][helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreatePropertyGroup( 
            /* [in] */ __RPC__in BSTR Name,
            /* [out][in] */ __RPC__inout LONG *dwIsoMode,
            /* [out][in] */ __RPC__inout LONG *dwRelMode,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedPropertyGroup **ppGroup) = 0;
        
        virtual /* [helpstring][helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ISharedPropertyGroup **ppGroup) = 0;
        
        virtual /* [helpstring][helpcontext][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISharedPropertyGroupManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISharedPropertyGroupManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISharedPropertyGroupManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISharedPropertyGroupManager * This,
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
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroupManager, CreatePropertyGroup)
        /* [helpstring][helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertyGroup )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [out][in] */ __RPC__inout LONG *dwIsoMode,
            /* [out][in] */ __RPC__inout LONG *dwRelMode,
            /* [out] */ __RPC__out VARIANT_BOOL *fExists,
            /* [retval][out] */ __RPC__deref_out_opt ISharedPropertyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroupManager, get_Group)
        /* [helpstring][helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ISharedPropertyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(ISharedPropertyGroupManager, get__NewEnum)
        /* [helpstring][helpcontext][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISharedPropertyGroupManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        END_INTERFACE
    } ISharedPropertyGroupManagerVtbl;

    interface ISharedPropertyGroupManager
    {
        CONST_VTBL struct ISharedPropertyGroupManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISharedPropertyGroupManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISharedPropertyGroupManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISharedPropertyGroupManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISharedPropertyGroupManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISharedPropertyGroupManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISharedPropertyGroupManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISharedPropertyGroupManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISharedPropertyGroupManager_CreatePropertyGroup(This,Name,dwIsoMode,dwRelMode,fExists,ppGroup)	\
    ( (This)->lpVtbl -> CreatePropertyGroup(This,Name,dwIsoMode,dwRelMode,fExists,ppGroup) ) 

#define ISharedPropertyGroupManager_get_Group(This,Name,ppGroup)	\
    ( (This)->lpVtbl -> get_Group(This,Name,ppGroup) ) 

#define ISharedPropertyGroupManager_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISharedPropertyGroupManager_INTERFACE_DEFINED__ */


#ifndef __IObjectConstruct_INTERFACE_DEFINED__
#define __IObjectConstruct_INTERFACE_DEFINED__

/* interface IObjectConstruct */
/* [uuid][helpstring][unique][object][local] */ 


EXTERN_C const IID IID_IObjectConstruct;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41C4F8B3-7439-11D2-98CB-00C04F8EE1C4")
    IObjectConstruct : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Construct( 
            /* [in] */ IDispatch *pCtorObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectConstructVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectConstruct * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectConstruct * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectConstruct * This);
        
        DECLSPEC_XFGVIRT(IObjectConstruct, Construct)
        HRESULT ( STDMETHODCALLTYPE *Construct )( 
            IObjectConstruct * This,
            /* [in] */ IDispatch *pCtorObj);
        
        END_INTERFACE
    } IObjectConstructVtbl;

    interface IObjectConstruct
    {
        CONST_VTBL struct IObjectConstructVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectConstruct_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectConstruct_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectConstruct_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectConstruct_Construct(This,pCtorObj)	\
    ( (This)->lpVtbl -> Construct(This,pCtorObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectConstruct_INTERFACE_DEFINED__ */


#ifndef __IObjectConstructString_INTERFACE_DEFINED__
#define __IObjectConstructString_INTERFACE_DEFINED__

/* interface IObjectConstructString */
/* [uuid][helpstring][dual][unique][object][local] */ 


EXTERN_C const IID IID_IObjectConstructString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41C4F8B2-7439-11D2-98CB-00C04F8EE1C4")
    IObjectConstructString : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConstructString( 
            /* [retval][out] */ BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectConstructStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectConstructString * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectConstructString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectConstructString * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IObjectConstructString * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IObjectConstructString * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IObjectConstructString * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IObjectConstructString * This,
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
        
        DECLSPEC_XFGVIRT(IObjectConstructString, get_ConstructString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConstructString )( 
            IObjectConstructString * This,
            /* [retval][out] */ BSTR *pVal);
        
        END_INTERFACE
    } IObjectConstructStringVtbl;

    interface IObjectConstructString
    {
        CONST_VTBL struct IObjectConstructStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectConstructString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectConstructString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectConstructString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectConstructString_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IObjectConstructString_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IObjectConstructString_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IObjectConstructString_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IObjectConstructString_get_ConstructString(This,pVal)	\
    ( (This)->lpVtbl -> get_ConstructString(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectConstructString_INTERFACE_DEFINED__ */


#ifndef __IObjectContextActivity_INTERFACE_DEFINED__
#define __IObjectContextActivity_INTERFACE_DEFINED__

/* interface IObjectContextActivity */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IObjectContextActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372afc-cae7-11cf-be81-00aa00a2fa25")
    IObjectContextActivity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetActivityId( 
            /* [out] */ GUID *pGUID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectContextActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectContextActivity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectContextActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectContextActivity * This);
        
        DECLSPEC_XFGVIRT(IObjectContextActivity, GetActivityId)
        HRESULT ( STDMETHODCALLTYPE *GetActivityId )( 
            IObjectContextActivity * This,
            /* [out] */ GUID *pGUID);
        
        END_INTERFACE
    } IObjectContextActivityVtbl;

    interface IObjectContextActivity
    {
        CONST_VTBL struct IObjectContextActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectContextActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectContextActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectContextActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectContextActivity_GetActivityId(This,pGUID)	\
    ( (This)->lpVtbl -> GetActivityId(This,pGUID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectContextActivity_INTERFACE_DEFINED__ */


#ifndef __IObjectContextInfo_INTERFACE_DEFINED__
#define __IObjectContextInfo_INTERFACE_DEFINED__

/* interface IObjectContextInfo */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IObjectContextInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75B52DDB-E8ED-11d1-93AD-00AA00BA3258")
    IObjectContextInfo : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE IsInTransaction( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransaction( 
            IUnknown **pptrans) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransactionId( 
            /* [out] */ GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityId( 
            /* [out] */ GUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContextId( 
            /* [out] */ GUID *pGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectContextInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectContextInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectContextInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectContextInfo * This);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, IsInTransaction)
        BOOL ( STDMETHODCALLTYPE *IsInTransaction )( 
            IObjectContextInfo * This);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetTransaction)
        HRESULT ( STDMETHODCALLTYPE *GetTransaction )( 
            IObjectContextInfo * This,
            IUnknown **pptrans);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetTransactionId)
        HRESULT ( STDMETHODCALLTYPE *GetTransactionId )( 
            IObjectContextInfo * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetActivityId)
        HRESULT ( STDMETHODCALLTYPE *GetActivityId )( 
            IObjectContextInfo * This,
            /* [out] */ GUID *pGUID);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetContextId)
        HRESULT ( STDMETHODCALLTYPE *GetContextId )( 
            IObjectContextInfo * This,
            /* [out] */ GUID *pGuid);
        
        END_INTERFACE
    } IObjectContextInfoVtbl;

    interface IObjectContextInfo
    {
        CONST_VTBL struct IObjectContextInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectContextInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectContextInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectContextInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectContextInfo_IsInTransaction(This)	\
    ( (This)->lpVtbl -> IsInTransaction(This) ) 

#define IObjectContextInfo_GetTransaction(This,pptrans)	\
    ( (This)->lpVtbl -> GetTransaction(This,pptrans) ) 

#define IObjectContextInfo_GetTransactionId(This,pGuid)	\
    ( (This)->lpVtbl -> GetTransactionId(This,pGuid) ) 

#define IObjectContextInfo_GetActivityId(This,pGUID)	\
    ( (This)->lpVtbl -> GetActivityId(This,pGUID) ) 

#define IObjectContextInfo_GetContextId(This,pGuid)	\
    ( (This)->lpVtbl -> GetContextId(This,pGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectContextInfo_INTERFACE_DEFINED__ */


#ifndef __IObjectContextInfo2_INTERFACE_DEFINED__
#define __IObjectContextInfo2_INTERFACE_DEFINED__

/* interface IObjectContextInfo2 */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IObjectContextInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("594BE71A-4BC4-438b-9197-CFD176248B09")
    IObjectContextInfo2 : public IObjectContextInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPartitionId( 
            /* [out] */ GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetApplicationId( 
            /* [out] */ GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetApplicationInstanceId( 
            /* [out] */ GUID *pGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectContextInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectContextInfo2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectContextInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectContextInfo2 * This);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, IsInTransaction)
        BOOL ( STDMETHODCALLTYPE *IsInTransaction )( 
            IObjectContextInfo2 * This);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetTransaction)
        HRESULT ( STDMETHODCALLTYPE *GetTransaction )( 
            IObjectContextInfo2 * This,
            IUnknown **pptrans);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetTransactionId)
        HRESULT ( STDMETHODCALLTYPE *GetTransactionId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetActivityId)
        HRESULT ( STDMETHODCALLTYPE *GetActivityId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGUID);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo, GetContextId)
        HRESULT ( STDMETHODCALLTYPE *GetContextId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo2, GetPartitionId)
        HRESULT ( STDMETHODCALLTYPE *GetPartitionId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo2, GetApplicationId)
        HRESULT ( STDMETHODCALLTYPE *GetApplicationId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IObjectContextInfo2, GetApplicationInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetApplicationInstanceId )( 
            IObjectContextInfo2 * This,
            /* [out] */ GUID *pGuid);
        
        END_INTERFACE
    } IObjectContextInfo2Vtbl;

    interface IObjectContextInfo2
    {
        CONST_VTBL struct IObjectContextInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectContextInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectContextInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectContextInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectContextInfo2_IsInTransaction(This)	\
    ( (This)->lpVtbl -> IsInTransaction(This) ) 

#define IObjectContextInfo2_GetTransaction(This,pptrans)	\
    ( (This)->lpVtbl -> GetTransaction(This,pptrans) ) 

#define IObjectContextInfo2_GetTransactionId(This,pGuid)	\
    ( (This)->lpVtbl -> GetTransactionId(This,pGuid) ) 

#define IObjectContextInfo2_GetActivityId(This,pGUID)	\
    ( (This)->lpVtbl -> GetActivityId(This,pGUID) ) 

#define IObjectContextInfo2_GetContextId(This,pGuid)	\
    ( (This)->lpVtbl -> GetContextId(This,pGuid) ) 


#define IObjectContextInfo2_GetPartitionId(This,pGuid)	\
    ( (This)->lpVtbl -> GetPartitionId(This,pGuid) ) 

#define IObjectContextInfo2_GetApplicationId(This,pGuid)	\
    ( (This)->lpVtbl -> GetApplicationId(This,pGuid) ) 

#define IObjectContextInfo2_GetApplicationInstanceId(This,pGuid)	\
    ( (This)->lpVtbl -> GetApplicationInstanceId(This,pGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectContextInfo2_INTERFACE_DEFINED__ */


#ifndef __ITransactionStatus_INTERFACE_DEFINED__
#define __ITransactionStatus_INTERFACE_DEFINED__

/* interface ITransactionStatus */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_ITransactionStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61F589E8-3724-4898-A0A4-664AE9E1D1B4")
    ITransactionStatus : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTransactionStatus( 
            HRESULT hrStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransactionStatus( 
            HRESULT *pHrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITransactionStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITransactionStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITransactionStatus * This);
        
        DECLSPEC_XFGVIRT(ITransactionStatus, SetTransactionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetTransactionStatus )( 
            ITransactionStatus * This,
            HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(ITransactionStatus, GetTransactionStatus)
        HRESULT ( STDMETHODCALLTYPE *GetTransactionStatus )( 
            ITransactionStatus * This,
            HRESULT *pHrStatus);
        
        END_INTERFACE
    } ITransactionStatusVtbl;

    interface ITransactionStatus
    {
        CONST_VTBL struct ITransactionStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionStatus_SetTransactionStatus(This,hrStatus)	\
    ( (This)->lpVtbl -> SetTransactionStatus(This,hrStatus) ) 

#define ITransactionStatus_GetTransactionStatus(This,pHrStatus)	\
    ( (This)->lpVtbl -> GetTransactionStatus(This,pHrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionStatus_INTERFACE_DEFINED__ */


#ifndef __IObjectContextTip_INTERFACE_DEFINED__
#define __IObjectContextTip_INTERFACE_DEFINED__

/* interface IObjectContextTip */
/* [object][uuid][unique][local] */ 


EXTERN_C const IID IID_IObjectContextTip;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92FD41CA-BAD9-11d2-9A2D-00C04F797BC9")
    IObjectContextTip : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTipUrl( 
            /* [retval][out] */ BSTR *pTipUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectContextTipVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjectContextTip * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjectContextTip * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjectContextTip * This);
        
        DECLSPEC_XFGVIRT(IObjectContextTip, GetTipUrl)
        HRESULT ( STDMETHODCALLTYPE *GetTipUrl )( 
            IObjectContextTip * This,
            /* [retval][out] */ BSTR *pTipUrl);
        
        END_INTERFACE
    } IObjectContextTipVtbl;

    interface IObjectContextTip
    {
        CONST_VTBL struct IObjectContextTipVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectContextTip_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectContextTip_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectContextTip_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectContextTip_GetTipUrl(This,pTipUrl)	\
    ( (This)->lpVtbl -> GetTipUrl(This,pTipUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectContextTip_INTERFACE_DEFINED__ */


#ifndef __IPlaybackControl_INTERFACE_DEFINED__
#define __IPlaybackControl_INTERFACE_DEFINED__

/* interface IPlaybackControl */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IPlaybackControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372afd-cae7-11cf-be81-00aa00a2fa25")
    IPlaybackControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FinalClientRetry( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FinalServerRetry( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPlaybackControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPlaybackControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPlaybackControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPlaybackControl * This);
        
        DECLSPEC_XFGVIRT(IPlaybackControl, FinalClientRetry)
        HRESULT ( STDMETHODCALLTYPE *FinalClientRetry )( 
            __RPC__in IPlaybackControl * This);
        
        DECLSPEC_XFGVIRT(IPlaybackControl, FinalServerRetry)
        HRESULT ( STDMETHODCALLTYPE *FinalServerRetry )( 
            __RPC__in IPlaybackControl * This);
        
        END_INTERFACE
    } IPlaybackControlVtbl;

    interface IPlaybackControl
    {
        CONST_VTBL struct IPlaybackControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPlaybackControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPlaybackControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPlaybackControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPlaybackControl_FinalClientRetry(This)	\
    ( (This)->lpVtbl -> FinalClientRetry(This) ) 

#define IPlaybackControl_FinalServerRetry(This)	\
    ( (This)->lpVtbl -> FinalServerRetry(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPlaybackControl_INTERFACE_DEFINED__ */


#ifndef __IGetContextProperties_INTERFACE_DEFINED__
#define __IGetContextProperties_INTERFACE_DEFINED__

/* interface IGetContextProperties */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IGetContextProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372af4-cae7-11cf-be81-00aa00a2fa25")
    IGetContextProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [retval][out] */ long *plCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *pProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumNames( 
            /* [retval][out] */ IEnumNames **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetContextPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGetContextProperties * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGetContextProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGetContextProperties * This);
        
        DECLSPEC_XFGVIRT(IGetContextProperties, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            IGetContextProperties * This,
            /* [retval][out] */ long *plCount);
        
        DECLSPEC_XFGVIRT(IGetContextProperties, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IGetContextProperties * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(IGetContextProperties, EnumNames)
        HRESULT ( STDMETHODCALLTYPE *EnumNames )( 
            IGetContextProperties * This,
            /* [retval][out] */ IEnumNames **ppenum);
        
        END_INTERFACE
    } IGetContextPropertiesVtbl;

    interface IGetContextProperties
    {
        CONST_VTBL struct IGetContextPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetContextProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetContextProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetContextProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetContextProperties_Count(This,plCount)	\
    ( (This)->lpVtbl -> Count(This,plCount) ) 

#define IGetContextProperties_GetProperty(This,name,pProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,name,pProperty) ) 

#define IGetContextProperties_EnumNames(This,ppenum)	\
    ( (This)->lpVtbl -> EnumNames(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetContextProperties_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0073 */
/* [local] */ 

typedef 
enum tagTransactionVote
    {
        TxCommit	= 0,
        TxAbort	= ( TxCommit + 1 ) 
    } 	TransactionVote;



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0073_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0073_v0_0_s_ifspec;

#ifndef __IContextState_INTERFACE_DEFINED__
#define __IContextState_INTERFACE_DEFINED__

/* interface IContextState */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IContextState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3C05E54B-A42A-11d2-AFC4-00C04F8EE1C4")
    IContextState : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDeactivateOnReturn( 
            VARIANT_BOOL bDeactivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeactivateOnReturn( 
            /* [out] */ VARIANT_BOOL *pbDeactivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMyTransactionVote( 
            TransactionVote txVote) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMyTransactionVote( 
            /* [out] */ TransactionVote *ptxVote) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IContextState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IContextState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IContextState * This);
        
        DECLSPEC_XFGVIRT(IContextState, SetDeactivateOnReturn)
        HRESULT ( STDMETHODCALLTYPE *SetDeactivateOnReturn )( 
            IContextState * This,
            VARIANT_BOOL bDeactivate);
        
        DECLSPEC_XFGVIRT(IContextState, GetDeactivateOnReturn)
        HRESULT ( STDMETHODCALLTYPE *GetDeactivateOnReturn )( 
            IContextState * This,
            /* [out] */ VARIANT_BOOL *pbDeactivate);
        
        DECLSPEC_XFGVIRT(IContextState, SetMyTransactionVote)
        HRESULT ( STDMETHODCALLTYPE *SetMyTransactionVote )( 
            IContextState * This,
            TransactionVote txVote);
        
        DECLSPEC_XFGVIRT(IContextState, GetMyTransactionVote)
        HRESULT ( STDMETHODCALLTYPE *GetMyTransactionVote )( 
            IContextState * This,
            /* [out] */ TransactionVote *ptxVote);
        
        END_INTERFACE
    } IContextStateVtbl;

    interface IContextState
    {
        CONST_VTBL struct IContextStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextState_SetDeactivateOnReturn(This,bDeactivate)	\
    ( (This)->lpVtbl -> SetDeactivateOnReturn(This,bDeactivate) ) 

#define IContextState_GetDeactivateOnReturn(This,pbDeactivate)	\
    ( (This)->lpVtbl -> GetDeactivateOnReturn(This,pbDeactivate) ) 

#define IContextState_SetMyTransactionVote(This,txVote)	\
    ( (This)->lpVtbl -> SetMyTransactionVote(This,txVote) ) 

#define IContextState_GetMyTransactionVote(This,ptxVote)	\
    ( (This)->lpVtbl -> GetMyTransactionVote(This,ptxVote) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextState_INTERFACE_DEFINED__ */


#ifndef __IPoolManager_INTERFACE_DEFINED__
#define __IPoolManager_INTERFACE_DEFINED__

/* interface IPoolManager */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IPoolManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a469861-5a91-43a0-99b6-d5e179bb0631")
    IPoolManager : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ShutdownPool( 
            /* [in] */ BSTR CLSIDOrProgID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPoolManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPoolManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPoolManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPoolManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPoolManager * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPoolManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPoolManager * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPoolManager * This,
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
        
        DECLSPEC_XFGVIRT(IPoolManager, ShutdownPool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShutdownPool )( 
            IPoolManager * This,
            /* [in] */ BSTR CLSIDOrProgID);
        
        END_INTERFACE
    } IPoolManagerVtbl;

    interface IPoolManager
    {
        CONST_VTBL struct IPoolManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPoolManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPoolManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPoolManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPoolManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPoolManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPoolManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPoolManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPoolManager_ShutdownPool(This,CLSIDOrProgID)	\
    ( (This)->lpVtbl -> ShutdownPool(This,CLSIDOrProgID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPoolManager_INTERFACE_DEFINED__ */


#ifndef __ISelectCOMLBServer_INTERFACE_DEFINED__
#define __ISelectCOMLBServer_INTERFACE_DEFINED__

/* interface ISelectCOMLBServer */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_ISelectCOMLBServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dcf443f4-3f8a-4872-b9f0-369a796d12d6")
    ISelectCOMLBServer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLBServer( 
            /* [in] */ IUnknown *pUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISelectCOMLBServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISelectCOMLBServer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISelectCOMLBServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISelectCOMLBServer * This);
        
        DECLSPEC_XFGVIRT(ISelectCOMLBServer, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            ISelectCOMLBServer * This);
        
        DECLSPEC_XFGVIRT(ISelectCOMLBServer, GetLBServer)
        HRESULT ( STDMETHODCALLTYPE *GetLBServer )( 
            ISelectCOMLBServer * This,
            /* [in] */ IUnknown *pUnk);
        
        END_INTERFACE
    } ISelectCOMLBServerVtbl;

    interface ISelectCOMLBServer
    {
        CONST_VTBL struct ISelectCOMLBServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISelectCOMLBServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISelectCOMLBServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISelectCOMLBServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISelectCOMLBServer_Init(This)	\
    ( (This)->lpVtbl -> Init(This) ) 

#define ISelectCOMLBServer_GetLBServer(This,pUnk)	\
    ( (This)->lpVtbl -> GetLBServer(This,pUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISelectCOMLBServer_INTERFACE_DEFINED__ */


#ifndef __ICOMLBArguments_INTERFACE_DEFINED__
#define __ICOMLBArguments_INTERFACE_DEFINED__

/* interface ICOMLBArguments */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_ICOMLBArguments;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3a0f150f-8ee5-4b94-b40e-aef2f9e42ed2")
    ICOMLBArguments : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCLSID( 
            /* [out] */ CLSID *pCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCLSID( 
            /* [in] */ CLSID *pCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMachineName( 
            /* [in] */ ULONG cchSvr,
            /* [annotation][max_is][out] */ 
            _Out_writes_(cchSvr)  WCHAR szServerName[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMachineName( 
            /* [in] */ ULONG cchSvr,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSvr)  WCHAR szServerName[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICOMLBArgumentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICOMLBArguments * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICOMLBArguments * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICOMLBArguments * This);
        
        DECLSPEC_XFGVIRT(ICOMLBArguments, GetCLSID)
        HRESULT ( STDMETHODCALLTYPE *GetCLSID )( 
            ICOMLBArguments * This,
            /* [out] */ CLSID *pCLSID);
        
        DECLSPEC_XFGVIRT(ICOMLBArguments, SetCLSID)
        HRESULT ( STDMETHODCALLTYPE *SetCLSID )( 
            ICOMLBArguments * This,
            /* [in] */ CLSID *pCLSID);
        
        DECLSPEC_XFGVIRT(ICOMLBArguments, GetMachineName)
        HRESULT ( STDMETHODCALLTYPE *GetMachineName )( 
            ICOMLBArguments * This,
            /* [in] */ ULONG cchSvr,
            /* [annotation][max_is][out] */ 
            _Out_writes_(cchSvr)  WCHAR szServerName[  ]);
        
        DECLSPEC_XFGVIRT(ICOMLBArguments, SetMachineName)
        HRESULT ( STDMETHODCALLTYPE *SetMachineName )( 
            ICOMLBArguments * This,
            /* [in] */ ULONG cchSvr,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSvr)  WCHAR szServerName[  ]);
        
        END_INTERFACE
    } ICOMLBArgumentsVtbl;

    interface ICOMLBArguments
    {
        CONST_VTBL struct ICOMLBArgumentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICOMLBArguments_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICOMLBArguments_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICOMLBArguments_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICOMLBArguments_GetCLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> GetCLSID(This,pCLSID) ) 

#define ICOMLBArguments_SetCLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> SetCLSID(This,pCLSID) ) 

#define ICOMLBArguments_GetMachineName(This,cchSvr,szServerName)	\
    ( (This)->lpVtbl -> GetMachineName(This,cchSvr,szServerName) ) 

#define ICOMLBArguments_SetMachineName(This,cchSvr,szServerName)	\
    ( (This)->lpVtbl -> SetMachineName(This,cchSvr,szServerName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICOMLBArguments_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0077 */
/* [local] */ 

#if (_WIN32_WINNT >= 0x0500)
#define GetObjectContext(ppIOC) (CoGetObjectContext(IID_IObjectContext, (void **) (ppIOC)) == S_OK ? S_OK : CONTEXT_E_NOCONTEXT)
#else
extern HRESULT __cdecl GetObjectContext (IObjectContext** ppInstanceContext);
#endif
EXTERN_C HRESULT __stdcall CoCreateActivity(IUnknown* pIUnknown, REFIID riid, void** ppObj );
EXTERN_C HRESULT __stdcall CoEnterServiceDomain(IUnknown* pConfigObject);
EXTERN_C void __stdcall CoLeaveServiceDomain(IUnknown *pUnkStatus);
EXTERN_C HRESULT __stdcall GetManagedExtensions(DWORD* dwExts);
extern void* __cdecl SafeRef(REFIID rid, IUnknown* pUnk);
extern HRESULT __cdecl RecycleSurrogate(long lReasonCode);



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0077_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0077_v0_0_s_ifspec;

#ifndef __ICrmLogControl_INTERFACE_DEFINED__
#define __ICrmLogControl_INTERFACE_DEFINED__

/* interface ICrmLogControl */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmLogControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A0E174B3-D26E-11d2-8F84-00805FC7BCD9")
    ICrmLogControl : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransactionUOW( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterCompensator( 
            /* [in] */ __RPC__in LPCWSTR lpcwstrProgIdCompensator,
            /* [in] */ __RPC__in LPCWSTR lpcwstrDescription,
            /* [in] */ LONG lCrmRegFlags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WriteLogRecordVariants( 
            /* [in] */ __RPC__in VARIANT *pLogRecord) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ForceLog( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ForgetLogRecord( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ForceTransactionToAbort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteLogRecord( 
            /* [size_is][in] */ __RPC__in_ecount_full(cBlob) BLOB rgBlob[  ],
            /* [in] */ ULONG cBlob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmLogControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmLogControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmLogControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmLogControl * This);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, get_TransactionUOW)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionUOW )( 
            __RPC__in ICrmLogControl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, RegisterCompensator)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterCompensator )( 
            __RPC__in ICrmLogControl * This,
            /* [in] */ __RPC__in LPCWSTR lpcwstrProgIdCompensator,
            /* [in] */ __RPC__in LPCWSTR lpcwstrDescription,
            /* [in] */ LONG lCrmRegFlags);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, WriteLogRecordVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WriteLogRecordVariants )( 
            __RPC__in ICrmLogControl * This,
            /* [in] */ __RPC__in VARIANT *pLogRecord);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, ForceLog)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ForceLog )( 
            __RPC__in ICrmLogControl * This);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, ForgetLogRecord)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ForgetLogRecord )( 
            __RPC__in ICrmLogControl * This);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, ForceTransactionToAbort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ForceTransactionToAbort )( 
            __RPC__in ICrmLogControl * This);
        
        DECLSPEC_XFGVIRT(ICrmLogControl, WriteLogRecord)
        HRESULT ( STDMETHODCALLTYPE *WriteLogRecord )( 
            __RPC__in ICrmLogControl * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cBlob) BLOB rgBlob[  ],
            /* [in] */ ULONG cBlob);
        
        END_INTERFACE
    } ICrmLogControlVtbl;

    interface ICrmLogControl
    {
        CONST_VTBL struct ICrmLogControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmLogControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmLogControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmLogControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmLogControl_get_TransactionUOW(This,pVal)	\
    ( (This)->lpVtbl -> get_TransactionUOW(This,pVal) ) 

#define ICrmLogControl_RegisterCompensator(This,lpcwstrProgIdCompensator,lpcwstrDescription,lCrmRegFlags)	\
    ( (This)->lpVtbl -> RegisterCompensator(This,lpcwstrProgIdCompensator,lpcwstrDescription,lCrmRegFlags) ) 

#define ICrmLogControl_WriteLogRecordVariants(This,pLogRecord)	\
    ( (This)->lpVtbl -> WriteLogRecordVariants(This,pLogRecord) ) 

#define ICrmLogControl_ForceLog(This)	\
    ( (This)->lpVtbl -> ForceLog(This) ) 

#define ICrmLogControl_ForgetLogRecord(This)	\
    ( (This)->lpVtbl -> ForgetLogRecord(This) ) 

#define ICrmLogControl_ForceTransactionToAbort(This)	\
    ( (This)->lpVtbl -> ForceTransactionToAbort(This) ) 

#define ICrmLogControl_WriteLogRecord(This,rgBlob,cBlob)	\
    ( (This)->lpVtbl -> WriteLogRecord(This,rgBlob,cBlob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmLogControl_INTERFACE_DEFINED__ */


#ifndef __ICrmCompensatorVariants_INTERFACE_DEFINED__
#define __ICrmCompensatorVariants_INTERFACE_DEFINED__

/* interface ICrmCompensatorVariants */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmCompensatorVariants;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0BAF8E4-7804-11d1-82E9-00A0C91EEDE9")
    ICrmCompensatorVariants : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetLogControlVariants( 
            /* [in] */ __RPC__in_opt ICrmLogControl *pLogControl) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BeginPrepareVariants( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PrepareRecordVariants( 
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EndPrepareVariants( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOkToPrepare) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BeginCommitVariants( 
            /* [in] */ VARIANT_BOOL bRecovery) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CommitRecordVariants( 
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EndCommitVariants( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BeginAbortVariants( 
            /* [in] */ VARIANT_BOOL bRecovery) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AbortRecordVariants( 
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EndAbortVariants( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmCompensatorVariantsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmCompensatorVariants * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmCompensatorVariants * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, SetLogControlVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetLogControlVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ __RPC__in_opt ICrmLogControl *pLogControl);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, BeginPrepareVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BeginPrepareVariants )( 
            __RPC__in ICrmCompensatorVariants * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, PrepareRecordVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PrepareRecordVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, EndPrepareVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EndPrepareVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOkToPrepare);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, BeginCommitVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BeginCommitVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ VARIANT_BOOL bRecovery);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, CommitRecordVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CommitRecordVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, EndCommitVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EndCommitVariants )( 
            __RPC__in ICrmCompensatorVariants * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, BeginAbortVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BeginAbortVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ VARIANT_BOOL bRecovery);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, AbortRecordVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AbortRecordVariants )( 
            __RPC__in ICrmCompensatorVariants * This,
            /* [in] */ __RPC__in VARIANT *pLogRecord,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensatorVariants, EndAbortVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EndAbortVariants )( 
            __RPC__in ICrmCompensatorVariants * This);
        
        END_INTERFACE
    } ICrmCompensatorVariantsVtbl;

    interface ICrmCompensatorVariants
    {
        CONST_VTBL struct ICrmCompensatorVariantsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmCompensatorVariants_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmCompensatorVariants_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmCompensatorVariants_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmCompensatorVariants_SetLogControlVariants(This,pLogControl)	\
    ( (This)->lpVtbl -> SetLogControlVariants(This,pLogControl) ) 

#define ICrmCompensatorVariants_BeginPrepareVariants(This)	\
    ( (This)->lpVtbl -> BeginPrepareVariants(This) ) 

#define ICrmCompensatorVariants_PrepareRecordVariants(This,pLogRecord,pbForget)	\
    ( (This)->lpVtbl -> PrepareRecordVariants(This,pLogRecord,pbForget) ) 

#define ICrmCompensatorVariants_EndPrepareVariants(This,pbOkToPrepare)	\
    ( (This)->lpVtbl -> EndPrepareVariants(This,pbOkToPrepare) ) 

#define ICrmCompensatorVariants_BeginCommitVariants(This,bRecovery)	\
    ( (This)->lpVtbl -> BeginCommitVariants(This,bRecovery) ) 

#define ICrmCompensatorVariants_CommitRecordVariants(This,pLogRecord,pbForget)	\
    ( (This)->lpVtbl -> CommitRecordVariants(This,pLogRecord,pbForget) ) 

#define ICrmCompensatorVariants_EndCommitVariants(This)	\
    ( (This)->lpVtbl -> EndCommitVariants(This) ) 

#define ICrmCompensatorVariants_BeginAbortVariants(This,bRecovery)	\
    ( (This)->lpVtbl -> BeginAbortVariants(This,bRecovery) ) 

#define ICrmCompensatorVariants_AbortRecordVariants(This,pLogRecord,pbForget)	\
    ( (This)->lpVtbl -> AbortRecordVariants(This,pLogRecord,pbForget) ) 

#define ICrmCompensatorVariants_EndAbortVariants(This)	\
    ( (This)->lpVtbl -> EndAbortVariants(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmCompensatorVariants_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0079 */
/* [local] */ 

#ifndef _tagCrmLogRecordRead_
#define _tagCrmLogRecordRead_
typedef struct tagCrmLogRecordRead
    {
    DWORD dwCrmFlags;
    DWORD dwSequenceNumber;
    BLOB blobUserData;
    } 	CrmLogRecordRead;

#endif // _tagCrmLogRecordRead_


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0079_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0079_v0_0_s_ifspec;

#ifndef __ICrmCompensator_INTERFACE_DEFINED__
#define __ICrmCompensator_INTERFACE_DEFINED__

/* interface ICrmCompensator */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmCompensator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BBC01830-8D3B-11d1-82EC-00A0C91EEDE9")
    ICrmCompensator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLogControl( 
            /* [in] */ __RPC__in_opt ICrmLogControl *pLogControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginPrepare( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrepareRecord( 
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndPrepare( 
            /* [retval][out] */ __RPC__out BOOL *pfOkToPrepare) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginCommit( 
            /* [in] */ BOOL fRecovery) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitRecord( 
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndCommit( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginAbort( 
            /* [in] */ BOOL fRecovery) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AbortRecord( 
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndAbort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmCompensatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmCompensator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmCompensator * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, SetLogControl)
        HRESULT ( STDMETHODCALLTYPE *SetLogControl )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ __RPC__in_opt ICrmLogControl *pLogControl);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, BeginPrepare)
        HRESULT ( STDMETHODCALLTYPE *BeginPrepare )( 
            __RPC__in ICrmCompensator * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, PrepareRecord)
        HRESULT ( STDMETHODCALLTYPE *PrepareRecord )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, EndPrepare)
        HRESULT ( STDMETHODCALLTYPE *EndPrepare )( 
            __RPC__in ICrmCompensator * This,
            /* [retval][out] */ __RPC__out BOOL *pfOkToPrepare);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, BeginCommit)
        HRESULT ( STDMETHODCALLTYPE *BeginCommit )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ BOOL fRecovery);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, CommitRecord)
        HRESULT ( STDMETHODCALLTYPE *CommitRecord )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, EndCommit)
        HRESULT ( STDMETHODCALLTYPE *EndCommit )( 
            __RPC__in ICrmCompensator * This);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, BeginAbort)
        HRESULT ( STDMETHODCALLTYPE *BeginAbort )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ BOOL fRecovery);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, AbortRecord)
        HRESULT ( STDMETHODCALLTYPE *AbortRecord )( 
            __RPC__in ICrmCompensator * This,
            /* [in] */ CrmLogRecordRead crmLogRec,
            /* [retval][out] */ __RPC__out BOOL *pfForget);
        
        DECLSPEC_XFGVIRT(ICrmCompensator, EndAbort)
        HRESULT ( STDMETHODCALLTYPE *EndAbort )( 
            __RPC__in ICrmCompensator * This);
        
        END_INTERFACE
    } ICrmCompensatorVtbl;

    interface ICrmCompensator
    {
        CONST_VTBL struct ICrmCompensatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmCompensator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmCompensator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmCompensator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmCompensator_SetLogControl(This,pLogControl)	\
    ( (This)->lpVtbl -> SetLogControl(This,pLogControl) ) 

#define ICrmCompensator_BeginPrepare(This)	\
    ( (This)->lpVtbl -> BeginPrepare(This) ) 

#define ICrmCompensator_PrepareRecord(This,crmLogRec,pfForget)	\
    ( (This)->lpVtbl -> PrepareRecord(This,crmLogRec,pfForget) ) 

#define ICrmCompensator_EndPrepare(This,pfOkToPrepare)	\
    ( (This)->lpVtbl -> EndPrepare(This,pfOkToPrepare) ) 

#define ICrmCompensator_BeginCommit(This,fRecovery)	\
    ( (This)->lpVtbl -> BeginCommit(This,fRecovery) ) 

#define ICrmCompensator_CommitRecord(This,crmLogRec,pfForget)	\
    ( (This)->lpVtbl -> CommitRecord(This,crmLogRec,pfForget) ) 

#define ICrmCompensator_EndCommit(This)	\
    ( (This)->lpVtbl -> EndCommit(This) ) 

#define ICrmCompensator_BeginAbort(This,fRecovery)	\
    ( (This)->lpVtbl -> BeginAbort(This,fRecovery) ) 

#define ICrmCompensator_AbortRecord(This,crmLogRec,pfForget)	\
    ( (This)->lpVtbl -> AbortRecord(This,crmLogRec,pfForget) ) 

#define ICrmCompensator_EndAbort(This)	\
    ( (This)->lpVtbl -> EndAbort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmCompensator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0080 */
/* [local] */ 

#ifndef _tagCrmTransactionState_
#define _tagCrmTransactionState_
typedef 
enum tagCrmTransactionState
    {
        TxState_Active	= 0,
        TxState_Committed	= ( TxState_Active + 1 ) ,
        TxState_Aborted	= ( TxState_Committed + 1 ) ,
        TxState_Indoubt	= ( TxState_Aborted + 1 ) 
    } 	CrmTransactionState;

#endif // _tagCrmTransactionState_


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0080_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0080_v0_0_s_ifspec;

#ifndef __ICrmMonitorLogRecords_INTERFACE_DEFINED__
#define __ICrmMonitorLogRecords_INTERFACE_DEFINED__

/* interface ICrmMonitorLogRecords */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmMonitorLogRecords;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70C8E441-C7ED-11d1-82FB-00A0C91EEDE9")
    ICrmMonitorLogRecords : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransactionState( 
            /* [retval][out] */ __RPC__out CrmTransactionState *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StructuredRecords( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLogRecord( 
            /* [in] */ DWORD dwIndex,
            /* [out][in] */ __RPC__inout CrmLogRecordRead *pCrmLogRec) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLogRecordVariants( 
            /* [in] */ VARIANT IndexNumber,
            /* [retval][out] */ __RPC__out LPVARIANT pLogRecord) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmMonitorLogRecordsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmMonitorLogRecords * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmMonitorLogRecords * This);
        
        DECLSPEC_XFGVIRT(ICrmMonitorLogRecords, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ICrmMonitorLogRecords, get_TransactionState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionState )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [retval][out] */ __RPC__out CrmTransactionState *pVal);
        
        DECLSPEC_XFGVIRT(ICrmMonitorLogRecords, get_StructuredRecords)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StructuredRecords )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(ICrmMonitorLogRecords, GetLogRecord)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLogRecord )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [in] */ DWORD dwIndex,
            /* [out][in] */ __RPC__inout CrmLogRecordRead *pCrmLogRec);
        
        DECLSPEC_XFGVIRT(ICrmMonitorLogRecords, GetLogRecordVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLogRecordVariants )( 
            __RPC__in ICrmMonitorLogRecords * This,
            /* [in] */ VARIANT IndexNumber,
            /* [retval][out] */ __RPC__out LPVARIANT pLogRecord);
        
        END_INTERFACE
    } ICrmMonitorLogRecordsVtbl;

    interface ICrmMonitorLogRecords
    {
        CONST_VTBL struct ICrmMonitorLogRecordsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmMonitorLogRecords_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmMonitorLogRecords_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmMonitorLogRecords_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmMonitorLogRecords_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define ICrmMonitorLogRecords_get_TransactionState(This,pVal)	\
    ( (This)->lpVtbl -> get_TransactionState(This,pVal) ) 

#define ICrmMonitorLogRecords_get_StructuredRecords(This,pVal)	\
    ( (This)->lpVtbl -> get_StructuredRecords(This,pVal) ) 

#define ICrmMonitorLogRecords_GetLogRecord(This,dwIndex,pCrmLogRec)	\
    ( (This)->lpVtbl -> GetLogRecord(This,dwIndex,pCrmLogRec) ) 

#define ICrmMonitorLogRecords_GetLogRecordVariants(This,IndexNumber,pLogRecord)	\
    ( (This)->lpVtbl -> GetLogRecordVariants(This,IndexNumber,pLogRecord) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmMonitorLogRecords_INTERFACE_DEFINED__ */


#ifndef __ICrmMonitorClerks_INTERFACE_DEFINED__
#define __ICrmMonitorClerks_INTERFACE_DEFINED__

/* interface ICrmMonitorClerks */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICrmMonitorClerks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70C8E442-C7ED-11d1-82FB-00A0C91EEDE9")
    ICrmMonitorClerks : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
        virtual /* [restricted][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProgIdCompensator( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Description( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TransactionUOW( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ActivityId( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmMonitorClerksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmMonitorClerks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmMonitorClerks * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICrmMonitorClerks * This,
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
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, get__NewEnum)
        /* [restricted][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *pVal);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, ProgIdCompensator)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ProgIdCompensator )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, Description)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Description )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, TransactionUOW)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TransactionUOW )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        DECLSPEC_XFGVIRT(ICrmMonitorClerks, ActivityId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ActivityId )( 
            __RPC__in ICrmMonitorClerks * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        END_INTERFACE
    } ICrmMonitorClerksVtbl;

    interface ICrmMonitorClerks
    {
        CONST_VTBL struct ICrmMonitorClerksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmMonitorClerks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmMonitorClerks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmMonitorClerks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmMonitorClerks_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICrmMonitorClerks_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICrmMonitorClerks_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICrmMonitorClerks_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICrmMonitorClerks_Item(This,Index,pItem)	\
    ( (This)->lpVtbl -> Item(This,Index,pItem) ) 

#define ICrmMonitorClerks_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#define ICrmMonitorClerks_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define ICrmMonitorClerks_ProgIdCompensator(This,Index,pItem)	\
    ( (This)->lpVtbl -> ProgIdCompensator(This,Index,pItem) ) 

#define ICrmMonitorClerks_Description(This,Index,pItem)	\
    ( (This)->lpVtbl -> Description(This,Index,pItem) ) 

#define ICrmMonitorClerks_TransactionUOW(This,Index,pItem)	\
    ( (This)->lpVtbl -> TransactionUOW(This,Index,pItem) ) 

#define ICrmMonitorClerks_ActivityId(This,Index,pItem)	\
    ( (This)->lpVtbl -> ActivityId(This,Index,pItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmMonitorClerks_INTERFACE_DEFINED__ */


#ifndef __ICrmMonitor_INTERFACE_DEFINED__
#define __ICrmMonitor_INTERFACE_DEFINED__

/* interface ICrmMonitor */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmMonitor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70C8E443-C7ED-11d1-82FB-00A0C91EEDE9")
    ICrmMonitor : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetClerks( 
            /* [retval][out] */ __RPC__deref_out_opt ICrmMonitorClerks **pClerks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE HoldClerk( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmMonitor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmMonitor * This);
        
        DECLSPEC_XFGVIRT(ICrmMonitor, GetClerks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetClerks )( 
            __RPC__in ICrmMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt ICrmMonitorClerks **pClerks);
        
        DECLSPEC_XFGVIRT(ICrmMonitor, HoldClerk)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HoldClerk )( 
            __RPC__in ICrmMonitor * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out LPVARIANT pItem);
        
        END_INTERFACE
    } ICrmMonitorVtbl;

    interface ICrmMonitor
    {
        CONST_VTBL struct ICrmMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmMonitor_GetClerks(This,pClerks)	\
    ( (This)->lpVtbl -> GetClerks(This,pClerks) ) 

#define ICrmMonitor_HoldClerk(This,Index,pItem)	\
    ( (This)->lpVtbl -> HoldClerk(This,Index,pItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmMonitor_INTERFACE_DEFINED__ */


#ifndef __ICrmFormatLogRecords_INTERFACE_DEFINED__
#define __ICrmFormatLogRecords_INTERFACE_DEFINED__

/* interface ICrmFormatLogRecords */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICrmFormatLogRecords;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9C51D821-C98B-11d1-82FB-00A0C91EEDE9")
    ICrmFormatLogRecords : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetColumnCount( 
            /* [out] */ __RPC__out long *plColumnCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetColumnHeaders( 
            /* [out] */ __RPC__out LPVARIANT pHeaders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetColumn( 
            /* [in] */ CrmLogRecordRead CrmLogRec,
            /* [out] */ __RPC__out LPVARIANT pFormattedLogRecord) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetColumnVariants( 
            /* [in] */ VARIANT LogRecord,
            /* [out] */ __RPC__out LPVARIANT pFormattedLogRecord) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICrmFormatLogRecordsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICrmFormatLogRecords * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICrmFormatLogRecords * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICrmFormatLogRecords * This);
        
        DECLSPEC_XFGVIRT(ICrmFormatLogRecords, GetColumnCount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumnCount )( 
            __RPC__in ICrmFormatLogRecords * This,
            /* [out] */ __RPC__out long *plColumnCount);
        
        DECLSPEC_XFGVIRT(ICrmFormatLogRecords, GetColumnHeaders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumnHeaders )( 
            __RPC__in ICrmFormatLogRecords * This,
            /* [out] */ __RPC__out LPVARIANT pHeaders);
        
        DECLSPEC_XFGVIRT(ICrmFormatLogRecords, GetColumn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            __RPC__in ICrmFormatLogRecords * This,
            /* [in] */ CrmLogRecordRead CrmLogRec,
            /* [out] */ __RPC__out LPVARIANT pFormattedLogRecord);
        
        DECLSPEC_XFGVIRT(ICrmFormatLogRecords, GetColumnVariants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumnVariants )( 
            __RPC__in ICrmFormatLogRecords * This,
            /* [in] */ VARIANT LogRecord,
            /* [out] */ __RPC__out LPVARIANT pFormattedLogRecord);
        
        END_INTERFACE
    } ICrmFormatLogRecordsVtbl;

    interface ICrmFormatLogRecords
    {
        CONST_VTBL struct ICrmFormatLogRecordsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICrmFormatLogRecords_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICrmFormatLogRecords_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICrmFormatLogRecords_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICrmFormatLogRecords_GetColumnCount(This,plColumnCount)	\
    ( (This)->lpVtbl -> GetColumnCount(This,plColumnCount) ) 

#define ICrmFormatLogRecords_GetColumnHeaders(This,pHeaders)	\
    ( (This)->lpVtbl -> GetColumnHeaders(This,pHeaders) ) 

#define ICrmFormatLogRecords_GetColumn(This,CrmLogRec,pFormattedLogRecord)	\
    ( (This)->lpVtbl -> GetColumn(This,CrmLogRec,pFormattedLogRecord) ) 

#define ICrmFormatLogRecords_GetColumnVariants(This,LogRecord,pFormattedLogRecord)	\
    ( (This)->lpVtbl -> GetColumnVariants(This,LogRecord,pFormattedLogRecord) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICrmFormatLogRecords_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0084 */
/* [local] */ 

typedef 
enum tagCSC_InheritanceConfig
    {
        CSC_Inherit	= 0,
        CSC_Ignore	= ( CSC_Inherit + 1 ) 
    } 	CSC_InheritanceConfig;

typedef 
enum tagCSC_ThreadPool
    {
        CSC_ThreadPoolNone	= 0,
        CSC_ThreadPoolInherit	= ( CSC_ThreadPoolNone + 1 ) ,
        CSC_STAThreadPool	= ( CSC_ThreadPoolInherit + 1 ) ,
        CSC_MTAThreadPool	= ( CSC_STAThreadPool + 1 ) 
    } 	CSC_ThreadPool;

typedef 
enum tagCSC_Binding
    {
        CSC_NoBinding	= 0,
        CSC_BindToPoolThread	= ( CSC_NoBinding + 1 ) 
    } 	CSC_Binding;

typedef 
enum tagCSC_TransactionConfig
    {
        CSC_NoTransaction	= 0,
        CSC_IfContainerIsTransactional	= ( CSC_NoTransaction + 1 ) ,
        CSC_CreateTransactionIfNecessary	= ( CSC_IfContainerIsTransactional + 1 ) ,
        CSC_NewTransaction	= ( CSC_CreateTransactionIfNecessary + 1 ) 
    } 	CSC_TransactionConfig;

typedef 
enum tagCSC_SynchronizationConfig
    {
        CSC_NoSynchronization	= 0,
        CSC_IfContainerIsSynchronized	= ( CSC_NoSynchronization + 1 ) ,
        CSC_NewSynchronizationIfNecessary	= ( CSC_IfContainerIsSynchronized + 1 ) ,
        CSC_NewSynchronization	= ( CSC_NewSynchronizationIfNecessary + 1 ) 
    } 	CSC_SynchronizationConfig;

typedef 
enum tagCSC_TrackerConfig
    {
        CSC_DontUseTracker	= 0,
        CSC_UseTracker	= ( CSC_DontUseTracker + 1 ) 
    } 	CSC_TrackerConfig;

typedef 
enum tagCSC_PartitionConfig
    {
        CSC_NoPartition	= 0,
        CSC_InheritPartition	= ( CSC_NoPartition + 1 ) ,
        CSC_NewPartition	= ( CSC_InheritPartition + 1 ) 
    } 	CSC_PartitionConfig;

typedef 
enum tagCSC_IISIntrinsicsConfig
    {
        CSC_NoIISIntrinsics	= 0,
        CSC_InheritIISIntrinsics	= ( CSC_NoIISIntrinsics + 1 ) 
    } 	CSC_IISIntrinsicsConfig;

typedef 
enum tagCSC_COMTIIntrinsicsConfig
    {
        CSC_NoCOMTIIntrinsics	= 0,
        CSC_InheritCOMTIIntrinsics	= ( CSC_NoCOMTIIntrinsics + 1 ) 
    } 	CSC_COMTIIntrinsicsConfig;

typedef 
enum tagCSC_SxsConfig
    {
        CSC_NoSxs	= 0,
        CSC_InheritSxs	= ( CSC_NoSxs + 1 ) ,
        CSC_NewSxs	= ( CSC_InheritSxs + 1 ) 
    } 	CSC_SxsConfig;



extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0084_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0084_v0_0_s_ifspec;

#ifndef __IServiceIISIntrinsicsConfig_INTERFACE_DEFINED__
#define __IServiceIISIntrinsicsConfig_INTERFACE_DEFINED__

/* interface IServiceIISIntrinsicsConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceIISIntrinsicsConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1a0cf920-d452-46f4-bc36-48118d54ea52")
    IServiceIISIntrinsicsConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IISIntrinsicsConfig( 
            /* [in] */ CSC_IISIntrinsicsConfig iisIntrinsicsConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceIISIntrinsicsConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceIISIntrinsicsConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceIISIntrinsicsConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceIISIntrinsicsConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceIISIntrinsicsConfig, IISIntrinsicsConfig)
        HRESULT ( STDMETHODCALLTYPE *IISIntrinsicsConfig )( 
            __RPC__in IServiceIISIntrinsicsConfig * This,
            /* [in] */ CSC_IISIntrinsicsConfig iisIntrinsicsConfig);
        
        END_INTERFACE
    } IServiceIISIntrinsicsConfigVtbl;

    interface IServiceIISIntrinsicsConfig
    {
        CONST_VTBL struct IServiceIISIntrinsicsConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceIISIntrinsicsConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceIISIntrinsicsConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceIISIntrinsicsConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceIISIntrinsicsConfig_IISIntrinsicsConfig(This,iisIntrinsicsConfig)	\
    ( (This)->lpVtbl -> IISIntrinsicsConfig(This,iisIntrinsicsConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceIISIntrinsicsConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceComTIIntrinsicsConfig_INTERFACE_DEFINED__
#define __IServiceComTIIntrinsicsConfig_INTERFACE_DEFINED__

/* interface IServiceComTIIntrinsicsConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceComTIIntrinsicsConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09e6831e-04e1-4ed4-9d0f-e8b168bafeaf")
    IServiceComTIIntrinsicsConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ComTIIntrinsicsConfig( 
            /* [in] */ CSC_COMTIIntrinsicsConfig comtiIntrinsicsConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceComTIIntrinsicsConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceComTIIntrinsicsConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceComTIIntrinsicsConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceComTIIntrinsicsConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceComTIIntrinsicsConfig, ComTIIntrinsicsConfig)
        HRESULT ( STDMETHODCALLTYPE *ComTIIntrinsicsConfig )( 
            __RPC__in IServiceComTIIntrinsicsConfig * This,
            /* [in] */ CSC_COMTIIntrinsicsConfig comtiIntrinsicsConfig);
        
        END_INTERFACE
    } IServiceComTIIntrinsicsConfigVtbl;

    interface IServiceComTIIntrinsicsConfig
    {
        CONST_VTBL struct IServiceComTIIntrinsicsConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceComTIIntrinsicsConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceComTIIntrinsicsConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceComTIIntrinsicsConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceComTIIntrinsicsConfig_ComTIIntrinsicsConfig(This,comtiIntrinsicsConfig)	\
    ( (This)->lpVtbl -> ComTIIntrinsicsConfig(This,comtiIntrinsicsConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceComTIIntrinsicsConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceSxsConfig_INTERFACE_DEFINED__
#define __IServiceSxsConfig_INTERFACE_DEFINED__

/* interface IServiceSxsConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceSxsConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7cd7379-f3f2-4634-811b-703281d73e08")
    IServiceSxsConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SxsConfig( 
            /* [in] */ CSC_SxsConfig scsConfig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SxsName( 
            /* [string][in] */ __RPC__in_string LPCWSTR szSxsName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SxsDirectory( 
            /* [string][in] */ __RPC__in_string LPCWSTR szSxsDirectory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceSxsConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceSxsConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceSxsConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceSxsConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceSxsConfig, SxsConfig)
        HRESULT ( STDMETHODCALLTYPE *SxsConfig )( 
            __RPC__in IServiceSxsConfig * This,
            /* [in] */ CSC_SxsConfig scsConfig);
        
        DECLSPEC_XFGVIRT(IServiceSxsConfig, SxsName)
        HRESULT ( STDMETHODCALLTYPE *SxsName )( 
            __RPC__in IServiceSxsConfig * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szSxsName);
        
        DECLSPEC_XFGVIRT(IServiceSxsConfig, SxsDirectory)
        HRESULT ( STDMETHODCALLTYPE *SxsDirectory )( 
            __RPC__in IServiceSxsConfig * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szSxsDirectory);
        
        END_INTERFACE
    } IServiceSxsConfigVtbl;

    interface IServiceSxsConfig
    {
        CONST_VTBL struct IServiceSxsConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceSxsConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceSxsConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceSxsConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceSxsConfig_SxsConfig(This,scsConfig)	\
    ( (This)->lpVtbl -> SxsConfig(This,scsConfig) ) 

#define IServiceSxsConfig_SxsName(This,szSxsName)	\
    ( (This)->lpVtbl -> SxsName(This,szSxsName) ) 

#define IServiceSxsConfig_SxsDirectory(This,szSxsDirectory)	\
    ( (This)->lpVtbl -> SxsDirectory(This,szSxsDirectory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceSxsConfig_INTERFACE_DEFINED__ */


#ifndef __ICheckSxsConfig_INTERFACE_DEFINED__
#define __ICheckSxsConfig_INTERFACE_DEFINED__

/* interface ICheckSxsConfig */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICheckSxsConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0FF5A96F-11FC-47d1-BAA6-25DD347E7242")
    ICheckSxsConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSameSxsConfig( 
            /* [string][in] */ LPCWSTR wszSxsName,
            /* [string][in] */ LPCWSTR wszSxsDirectory,
            /* [string][in] */ LPCWSTR wszSxsAppName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICheckSxsConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICheckSxsConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICheckSxsConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICheckSxsConfig * This);
        
        DECLSPEC_XFGVIRT(ICheckSxsConfig, IsSameSxsConfig)
        HRESULT ( STDMETHODCALLTYPE *IsSameSxsConfig )( 
            ICheckSxsConfig * This,
            /* [string][in] */ LPCWSTR wszSxsName,
            /* [string][in] */ LPCWSTR wszSxsDirectory,
            /* [string][in] */ LPCWSTR wszSxsAppName);
        
        END_INTERFACE
    } ICheckSxsConfigVtbl;

    interface ICheckSxsConfig
    {
        CONST_VTBL struct ICheckSxsConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICheckSxsConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICheckSxsConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICheckSxsConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICheckSxsConfig_IsSameSxsConfig(This,wszSxsName,wszSxsDirectory,wszSxsAppName)	\
    ( (This)->lpVtbl -> IsSameSxsConfig(This,wszSxsName,wszSxsDirectory,wszSxsAppName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICheckSxsConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceInheritanceConfig_INTERFACE_DEFINED__
#define __IServiceInheritanceConfig_INTERFACE_DEFINED__

/* interface IServiceInheritanceConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceInheritanceConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92186771-d3b4-4d77-a8ea-ee842d586f35")
    IServiceInheritanceConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ContainingContextTreatment( 
            /* [in] */ CSC_InheritanceConfig inheritanceConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceInheritanceConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceInheritanceConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceInheritanceConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceInheritanceConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceInheritanceConfig, ContainingContextTreatment)
        HRESULT ( STDMETHODCALLTYPE *ContainingContextTreatment )( 
            __RPC__in IServiceInheritanceConfig * This,
            /* [in] */ CSC_InheritanceConfig inheritanceConfig);
        
        END_INTERFACE
    } IServiceInheritanceConfigVtbl;

    interface IServiceInheritanceConfig
    {
        CONST_VTBL struct IServiceInheritanceConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceInheritanceConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceInheritanceConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceInheritanceConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceInheritanceConfig_ContainingContextTreatment(This,inheritanceConfig)	\
    ( (This)->lpVtbl -> ContainingContextTreatment(This,inheritanceConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceInheritanceConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceThreadPoolConfig_INTERFACE_DEFINED__
#define __IServiceThreadPoolConfig_INTERFACE_DEFINED__

/* interface IServiceThreadPoolConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceThreadPoolConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("186d89bc-f277-4bcc-80d5-4df7b836ef4a")
    IServiceThreadPoolConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SelectThreadPool( 
            /* [in] */ CSC_ThreadPool threadPool) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBindingInfo( 
            /* [in] */ CSC_Binding binding) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceThreadPoolConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceThreadPoolConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceThreadPoolConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceThreadPoolConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceThreadPoolConfig, SelectThreadPool)
        HRESULT ( STDMETHODCALLTYPE *SelectThreadPool )( 
            __RPC__in IServiceThreadPoolConfig * This,
            /* [in] */ CSC_ThreadPool threadPool);
        
        DECLSPEC_XFGVIRT(IServiceThreadPoolConfig, SetBindingInfo)
        HRESULT ( STDMETHODCALLTYPE *SetBindingInfo )( 
            __RPC__in IServiceThreadPoolConfig * This,
            /* [in] */ CSC_Binding binding);
        
        END_INTERFACE
    } IServiceThreadPoolConfigVtbl;

    interface IServiceThreadPoolConfig
    {
        CONST_VTBL struct IServiceThreadPoolConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceThreadPoolConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceThreadPoolConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceThreadPoolConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceThreadPoolConfig_SelectThreadPool(This,threadPool)	\
    ( (This)->lpVtbl -> SelectThreadPool(This,threadPool) ) 

#define IServiceThreadPoolConfig_SetBindingInfo(This,binding)	\
    ( (This)->lpVtbl -> SetBindingInfo(This,binding) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceThreadPoolConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceTransactionConfigBase_INTERFACE_DEFINED__
#define __IServiceTransactionConfigBase_INTERFACE_DEFINED__

/* interface IServiceTransactionConfigBase */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceTransactionConfigBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("772b3fbe-6ffd-42fb-b5f8-8f9b260f3810")
    IServiceTransactionConfigBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConfigureTransaction( 
            /* [in] */ CSC_TransactionConfig transactionConfig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsolationLevel( 
            /* [in] */ COMAdminTxIsolationLevelOptions option) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransactionTimeout( 
            /* [in] */ ULONG ulTimeoutSec) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringYourOwnTransaction( 
            /* [string][in] */ __RPC__in_string LPCWSTR szTipURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NewTransactionDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR szTxDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceTransactionConfigBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceTransactionConfigBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceTransactionConfigBase * This);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, ConfigureTransaction)
        HRESULT ( STDMETHODCALLTYPE *ConfigureTransaction )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [in] */ CSC_TransactionConfig transactionConfig);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, IsolationLevel)
        HRESULT ( STDMETHODCALLTYPE *IsolationLevel )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [in] */ COMAdminTxIsolationLevelOptions option);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, TransactionTimeout)
        HRESULT ( STDMETHODCALLTYPE *TransactionTimeout )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [in] */ ULONG ulTimeoutSec);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, BringYourOwnTransaction)
        HRESULT ( STDMETHODCALLTYPE *BringYourOwnTransaction )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szTipURL);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, NewTransactionDescription)
        HRESULT ( STDMETHODCALLTYPE *NewTransactionDescription )( 
            __RPC__in IServiceTransactionConfigBase * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szTxDesc);
        
        END_INTERFACE
    } IServiceTransactionConfigBaseVtbl;

    interface IServiceTransactionConfigBase
    {
        CONST_VTBL struct IServiceTransactionConfigBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceTransactionConfigBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceTransactionConfigBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceTransactionConfigBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceTransactionConfigBase_ConfigureTransaction(This,transactionConfig)	\
    ( (This)->lpVtbl -> ConfigureTransaction(This,transactionConfig) ) 

#define IServiceTransactionConfigBase_IsolationLevel(This,option)	\
    ( (This)->lpVtbl -> IsolationLevel(This,option) ) 

#define IServiceTransactionConfigBase_TransactionTimeout(This,ulTimeoutSec)	\
    ( (This)->lpVtbl -> TransactionTimeout(This,ulTimeoutSec) ) 

#define IServiceTransactionConfigBase_BringYourOwnTransaction(This,szTipURL)	\
    ( (This)->lpVtbl -> BringYourOwnTransaction(This,szTipURL) ) 

#define IServiceTransactionConfigBase_NewTransactionDescription(This,szTxDesc)	\
    ( (This)->lpVtbl -> NewTransactionDescription(This,szTxDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceTransactionConfigBase_INTERFACE_DEFINED__ */


#ifndef __IServiceTransactionConfig_INTERFACE_DEFINED__
#define __IServiceTransactionConfig_INTERFACE_DEFINED__

/* interface IServiceTransactionConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceTransactionConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59f4c2a3-d3d7-4a31-b6e4-6ab3177c50b9")
    IServiceTransactionConfig : public IServiceTransactionConfigBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConfigureBYOT( 
            /* [in] */ __RPC__in_opt ITransaction *pITxByot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceTransactionConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceTransactionConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceTransactionConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, ConfigureTransaction)
        HRESULT ( STDMETHODCALLTYPE *ConfigureTransaction )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [in] */ CSC_TransactionConfig transactionConfig);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, IsolationLevel)
        HRESULT ( STDMETHODCALLTYPE *IsolationLevel )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [in] */ COMAdminTxIsolationLevelOptions option);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, TransactionTimeout)
        HRESULT ( STDMETHODCALLTYPE *TransactionTimeout )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [in] */ ULONG ulTimeoutSec);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, BringYourOwnTransaction)
        HRESULT ( STDMETHODCALLTYPE *BringYourOwnTransaction )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szTipURL);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, NewTransactionDescription)
        HRESULT ( STDMETHODCALLTYPE *NewTransactionDescription )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [string][in] */ __RPC__in_string LPCWSTR szTxDesc);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfig, ConfigureBYOT)
        HRESULT ( STDMETHODCALLTYPE *ConfigureBYOT )( 
            __RPC__in IServiceTransactionConfig * This,
            /* [in] */ __RPC__in_opt ITransaction *pITxByot);
        
        END_INTERFACE
    } IServiceTransactionConfigVtbl;

    interface IServiceTransactionConfig
    {
        CONST_VTBL struct IServiceTransactionConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceTransactionConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceTransactionConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceTransactionConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceTransactionConfig_ConfigureTransaction(This,transactionConfig)	\
    ( (This)->lpVtbl -> ConfigureTransaction(This,transactionConfig) ) 

#define IServiceTransactionConfig_IsolationLevel(This,option)	\
    ( (This)->lpVtbl -> IsolationLevel(This,option) ) 

#define IServiceTransactionConfig_TransactionTimeout(This,ulTimeoutSec)	\
    ( (This)->lpVtbl -> TransactionTimeout(This,ulTimeoutSec) ) 

#define IServiceTransactionConfig_BringYourOwnTransaction(This,szTipURL)	\
    ( (This)->lpVtbl -> BringYourOwnTransaction(This,szTipURL) ) 

#define IServiceTransactionConfig_NewTransactionDescription(This,szTxDesc)	\
    ( (This)->lpVtbl -> NewTransactionDescription(This,szTxDesc) ) 


#define IServiceTransactionConfig_ConfigureBYOT(This,pITxByot)	\
    ( (This)->lpVtbl -> ConfigureBYOT(This,pITxByot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceTransactionConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceSysTxnConfig_INTERFACE_DEFINED__
#define __IServiceSysTxnConfig_INTERFACE_DEFINED__

/* interface IServiceSysTxnConfig */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IServiceSysTxnConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33CAF1A1-FCB8-472b-B45E-967448DED6D8")
    IServiceSysTxnConfig : public IServiceTransactionConfig
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConfigureBYOTSysTxn( 
            /* [in] */ ITransactionProxy *pTxProxy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceSysTxnConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IServiceSysTxnConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IServiceSysTxnConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IServiceSysTxnConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, ConfigureTransaction)
        HRESULT ( STDMETHODCALLTYPE *ConfigureTransaction )( 
            IServiceSysTxnConfig * This,
            /* [in] */ CSC_TransactionConfig transactionConfig);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, IsolationLevel)
        HRESULT ( STDMETHODCALLTYPE *IsolationLevel )( 
            IServiceSysTxnConfig * This,
            /* [in] */ COMAdminTxIsolationLevelOptions option);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, TransactionTimeout)
        HRESULT ( STDMETHODCALLTYPE *TransactionTimeout )( 
            IServiceSysTxnConfig * This,
            /* [in] */ ULONG ulTimeoutSec);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, BringYourOwnTransaction)
        HRESULT ( STDMETHODCALLTYPE *BringYourOwnTransaction )( 
            IServiceSysTxnConfig * This,
            /* [string][in] */ LPCWSTR szTipURL);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfigBase, NewTransactionDescription)
        HRESULT ( STDMETHODCALLTYPE *NewTransactionDescription )( 
            IServiceSysTxnConfig * This,
            /* [string][in] */ LPCWSTR szTxDesc);
        
        DECLSPEC_XFGVIRT(IServiceTransactionConfig, ConfigureBYOT)
        HRESULT ( STDMETHODCALLTYPE *ConfigureBYOT )( 
            IServiceSysTxnConfig * This,
            /* [in] */ ITransaction *pITxByot);
        
        DECLSPEC_XFGVIRT(IServiceSysTxnConfig, ConfigureBYOTSysTxn)
        HRESULT ( STDMETHODCALLTYPE *ConfigureBYOTSysTxn )( 
            IServiceSysTxnConfig * This,
            /* [in] */ ITransactionProxy *pTxProxy);
        
        END_INTERFACE
    } IServiceSysTxnConfigVtbl;

    interface IServiceSysTxnConfig
    {
        CONST_VTBL struct IServiceSysTxnConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceSysTxnConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceSysTxnConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceSysTxnConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceSysTxnConfig_ConfigureTransaction(This,transactionConfig)	\
    ( (This)->lpVtbl -> ConfigureTransaction(This,transactionConfig) ) 

#define IServiceSysTxnConfig_IsolationLevel(This,option)	\
    ( (This)->lpVtbl -> IsolationLevel(This,option) ) 

#define IServiceSysTxnConfig_TransactionTimeout(This,ulTimeoutSec)	\
    ( (This)->lpVtbl -> TransactionTimeout(This,ulTimeoutSec) ) 

#define IServiceSysTxnConfig_BringYourOwnTransaction(This,szTipURL)	\
    ( (This)->lpVtbl -> BringYourOwnTransaction(This,szTipURL) ) 

#define IServiceSysTxnConfig_NewTransactionDescription(This,szTxDesc)	\
    ( (This)->lpVtbl -> NewTransactionDescription(This,szTxDesc) ) 


#define IServiceSysTxnConfig_ConfigureBYOT(This,pITxByot)	\
    ( (This)->lpVtbl -> ConfigureBYOT(This,pITxByot) ) 


#define IServiceSysTxnConfig_ConfigureBYOTSysTxn(This,pTxProxy)	\
    ( (This)->lpVtbl -> ConfigureBYOTSysTxn(This,pTxProxy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceSysTxnConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceSynchronizationConfig_INTERFACE_DEFINED__
#define __IServiceSynchronizationConfig_INTERFACE_DEFINED__

/* interface IServiceSynchronizationConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceSynchronizationConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd880e81-6dce-4c58-af83-a208846c0030")
    IServiceSynchronizationConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConfigureSynchronization( 
            /* [in] */ CSC_SynchronizationConfig synchConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceSynchronizationConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceSynchronizationConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceSynchronizationConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceSynchronizationConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceSynchronizationConfig, ConfigureSynchronization)
        HRESULT ( STDMETHODCALLTYPE *ConfigureSynchronization )( 
            __RPC__in IServiceSynchronizationConfig * This,
            /* [in] */ CSC_SynchronizationConfig synchConfig);
        
        END_INTERFACE
    } IServiceSynchronizationConfigVtbl;

    interface IServiceSynchronizationConfig
    {
        CONST_VTBL struct IServiceSynchronizationConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceSynchronizationConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceSynchronizationConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceSynchronizationConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceSynchronizationConfig_ConfigureSynchronization(This,synchConfig)	\
    ( (This)->lpVtbl -> ConfigureSynchronization(This,synchConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceSynchronizationConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceTrackerConfig_INTERFACE_DEFINED__
#define __IServiceTrackerConfig_INTERFACE_DEFINED__

/* interface IServiceTrackerConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceTrackerConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6c3a3e1d-0ba6-4036-b76f-d0404db816c9")
    IServiceTrackerConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TrackerConfig( 
            /* [in] */ CSC_TrackerConfig trackerConfig,
            /* [string][in] */ __RPC__in_string LPCWSTR szTrackerAppName,
            /* [string][in] */ __RPC__in_string LPCWSTR szTrackerCtxName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceTrackerConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceTrackerConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceTrackerConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceTrackerConfig * This);
        
        DECLSPEC_XFGVIRT(IServiceTrackerConfig, TrackerConfig)
        HRESULT ( STDMETHODCALLTYPE *TrackerConfig )( 
            __RPC__in IServiceTrackerConfig * This,
            /* [in] */ CSC_TrackerConfig trackerConfig,
            /* [string][in] */ __RPC__in_string LPCWSTR szTrackerAppName,
            /* [string][in] */ __RPC__in_string LPCWSTR szTrackerCtxName);
        
        END_INTERFACE
    } IServiceTrackerConfigVtbl;

    interface IServiceTrackerConfig
    {
        CONST_VTBL struct IServiceTrackerConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceTrackerConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceTrackerConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceTrackerConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceTrackerConfig_TrackerConfig(This,trackerConfig,szTrackerAppName,szTrackerCtxName)	\
    ( (This)->lpVtbl -> TrackerConfig(This,trackerConfig,szTrackerAppName,szTrackerCtxName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceTrackerConfig_INTERFACE_DEFINED__ */


#ifndef __IServicePartitionConfig_INTERFACE_DEFINED__
#define __IServicePartitionConfig_INTERFACE_DEFINED__

/* interface IServicePartitionConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServicePartitionConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80182d03-5ea4-4831-ae97-55beffc2e590")
    IServicePartitionConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PartitionConfig( 
            /* [in] */ CSC_PartitionConfig partitionConfig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PartitionID( 
            /* [in] */ __RPC__in REFGUID guidPartitionID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServicePartitionConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServicePartitionConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServicePartitionConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServicePartitionConfig * This);
        
        DECLSPEC_XFGVIRT(IServicePartitionConfig, PartitionConfig)
        HRESULT ( STDMETHODCALLTYPE *PartitionConfig )( 
            __RPC__in IServicePartitionConfig * This,
            /* [in] */ CSC_PartitionConfig partitionConfig);
        
        DECLSPEC_XFGVIRT(IServicePartitionConfig, PartitionID)
        HRESULT ( STDMETHODCALLTYPE *PartitionID )( 
            __RPC__in IServicePartitionConfig * This,
            /* [in] */ __RPC__in REFGUID guidPartitionID);
        
        END_INTERFACE
    } IServicePartitionConfigVtbl;

    interface IServicePartitionConfig
    {
        CONST_VTBL struct IServicePartitionConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServicePartitionConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServicePartitionConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServicePartitionConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServicePartitionConfig_PartitionConfig(This,partitionConfig)	\
    ( (This)->lpVtbl -> PartitionConfig(This,partitionConfig) ) 

#define IServicePartitionConfig_PartitionID(This,guidPartitionID)	\
    ( (This)->lpVtbl -> PartitionID(This,guidPartitionID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServicePartitionConfig_INTERFACE_DEFINED__ */


#ifndef __IServiceCall_INTERFACE_DEFINED__
#define __IServiceCall_INTERFACE_DEFINED__

/* interface IServiceCall */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceCall;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BD3E2E12-42DD-40f4-A09A-95A50C58304B")
    IServiceCall : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCall( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceCallVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceCall * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceCall * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceCall * This);
        
        DECLSPEC_XFGVIRT(IServiceCall, OnCall)
        HRESULT ( STDMETHODCALLTYPE *OnCall )( 
            __RPC__in IServiceCall * This);
        
        END_INTERFACE
    } IServiceCallVtbl;

    interface IServiceCall
    {
        CONST_VTBL struct IServiceCallVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceCall_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceCall_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceCall_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceCall_OnCall(This)	\
    ( (This)->lpVtbl -> OnCall(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceCall_INTERFACE_DEFINED__ */


#ifndef __IAsyncErrorNotify_INTERFACE_DEFINED__
#define __IAsyncErrorNotify_INTERFACE_DEFINED__

/* interface IAsyncErrorNotify */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAsyncErrorNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FE6777FB-A674-4177-8F32-6D707E113484")
    IAsyncErrorNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnError( 
            HRESULT hr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncErrorNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAsyncErrorNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAsyncErrorNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAsyncErrorNotify * This);
        
        DECLSPEC_XFGVIRT(IAsyncErrorNotify, OnError)
        HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in IAsyncErrorNotify * This,
            HRESULT hr);
        
        END_INTERFACE
    } IAsyncErrorNotifyVtbl;

    interface IAsyncErrorNotify
    {
        CONST_VTBL struct IAsyncErrorNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncErrorNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncErrorNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncErrorNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncErrorNotify_OnError(This,hr)	\
    ( (This)->lpVtbl -> OnError(This,hr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncErrorNotify_INTERFACE_DEFINED__ */


#ifndef __IServiceActivity_INTERFACE_DEFINED__
#define __IServiceActivity_INTERFACE_DEFINED__

/* interface IServiceActivity */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IServiceActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("67532E0C-9E2F-4450-A354-035633944E17")
    IServiceActivity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCall( 
            /* [in] */ __RPC__in_opt IServiceCall *pIServiceCall) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsynchronousCall( 
            /* [in] */ __RPC__in_opt IServiceCall *pIServiceCall) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BindToCurrentThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnbindFromThread( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceActivity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IServiceActivity, SynchronousCall)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCall )( 
            __RPC__in IServiceActivity * This,
            /* [in] */ __RPC__in_opt IServiceCall *pIServiceCall);
        
        DECLSPEC_XFGVIRT(IServiceActivity, AsynchronousCall)
        HRESULT ( STDMETHODCALLTYPE *AsynchronousCall )( 
            __RPC__in IServiceActivity * This,
            /* [in] */ __RPC__in_opt IServiceCall *pIServiceCall);
        
        DECLSPEC_XFGVIRT(IServiceActivity, BindToCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *BindToCurrentThread )( 
            __RPC__in IServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IServiceActivity, UnbindFromThread)
        HRESULT ( STDMETHODCALLTYPE *UnbindFromThread )( 
            __RPC__in IServiceActivity * This);
        
        END_INTERFACE
    } IServiceActivityVtbl;

    interface IServiceActivity
    {
        CONST_VTBL struct IServiceActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceActivity_SynchronousCall(This,pIServiceCall)	\
    ( (This)->lpVtbl -> SynchronousCall(This,pIServiceCall) ) 

#define IServiceActivity_AsynchronousCall(This,pIServiceCall)	\
    ( (This)->lpVtbl -> AsynchronousCall(This,pIServiceCall) ) 

#define IServiceActivity_BindToCurrentThread(This)	\
    ( (This)->lpVtbl -> BindToCurrentThread(This) ) 

#define IServiceActivity_UnbindFromThread(This)	\
    ( (This)->lpVtbl -> UnbindFromThread(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceActivity_INTERFACE_DEFINED__ */


#ifndef __IThreadPoolKnobs_INTERFACE_DEFINED__
#define __IThreadPoolKnobs_INTERFACE_DEFINED__

/* interface IThreadPoolKnobs */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IThreadPoolKnobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372af7-cae7-11cf-be81-00aa00a2fa25")
    IThreadPoolKnobs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMaxThreads( 
            long *plcMaxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentThreads( 
            long *plcCurrentThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxThreads( 
            long lcMaxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeleteDelay( 
            long *pmsecDeleteDelay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeleteDelay( 
            long msecDeleteDelay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxQueuedRequests( 
            long *plcMaxQueuedRequests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentQueuedRequests( 
            long *plcCurrentQueuedRequests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxQueuedRequests( 
            long lcMaxQueuedRequests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMinThreads( 
            long lcMinThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetQueueDepth( 
            long lcQueueDepth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThreadPoolKnobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IThreadPoolKnobs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, GetMaxThreads)
        HRESULT ( STDMETHODCALLTYPE *GetMaxThreads )( 
            IThreadPoolKnobs * This,
            long *plcMaxThreads);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, GetCurrentThreads)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentThreads )( 
            IThreadPoolKnobs * This,
            long *plcCurrentThreads);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, SetMaxThreads)
        HRESULT ( STDMETHODCALLTYPE *SetMaxThreads )( 
            IThreadPoolKnobs * This,
            long lcMaxThreads);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, GetDeleteDelay)
        HRESULT ( STDMETHODCALLTYPE *GetDeleteDelay )( 
            IThreadPoolKnobs * This,
            long *pmsecDeleteDelay);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, SetDeleteDelay)
        HRESULT ( STDMETHODCALLTYPE *SetDeleteDelay )( 
            IThreadPoolKnobs * This,
            long msecDeleteDelay);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, GetMaxQueuedRequests)
        HRESULT ( STDMETHODCALLTYPE *GetMaxQueuedRequests )( 
            IThreadPoolKnobs * This,
            long *plcMaxQueuedRequests);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, GetCurrentQueuedRequests)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentQueuedRequests )( 
            IThreadPoolKnobs * This,
            long *plcCurrentQueuedRequests);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, SetMaxQueuedRequests)
        HRESULT ( STDMETHODCALLTYPE *SetMaxQueuedRequests )( 
            IThreadPoolKnobs * This,
            long lcMaxQueuedRequests);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, SetMinThreads)
        HRESULT ( STDMETHODCALLTYPE *SetMinThreads )( 
            IThreadPoolKnobs * This,
            long lcMinThreads);
        
        DECLSPEC_XFGVIRT(IThreadPoolKnobs, SetQueueDepth)
        HRESULT ( STDMETHODCALLTYPE *SetQueueDepth )( 
            IThreadPoolKnobs * This,
            long lcQueueDepth);
        
        END_INTERFACE
    } IThreadPoolKnobsVtbl;

    interface IThreadPoolKnobs
    {
        CONST_VTBL struct IThreadPoolKnobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThreadPoolKnobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThreadPoolKnobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThreadPoolKnobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThreadPoolKnobs_GetMaxThreads(This,plcMaxThreads)	\
    ( (This)->lpVtbl -> GetMaxThreads(This,plcMaxThreads) ) 

#define IThreadPoolKnobs_GetCurrentThreads(This,plcCurrentThreads)	\
    ( (This)->lpVtbl -> GetCurrentThreads(This,plcCurrentThreads) ) 

#define IThreadPoolKnobs_SetMaxThreads(This,lcMaxThreads)	\
    ( (This)->lpVtbl -> SetMaxThreads(This,lcMaxThreads) ) 

#define IThreadPoolKnobs_GetDeleteDelay(This,pmsecDeleteDelay)	\
    ( (This)->lpVtbl -> GetDeleteDelay(This,pmsecDeleteDelay) ) 

#define IThreadPoolKnobs_SetDeleteDelay(This,msecDeleteDelay)	\
    ( (This)->lpVtbl -> SetDeleteDelay(This,msecDeleteDelay) ) 

#define IThreadPoolKnobs_GetMaxQueuedRequests(This,plcMaxQueuedRequests)	\
    ( (This)->lpVtbl -> GetMaxQueuedRequests(This,plcMaxQueuedRequests) ) 

#define IThreadPoolKnobs_GetCurrentQueuedRequests(This,plcCurrentQueuedRequests)	\
    ( (This)->lpVtbl -> GetCurrentQueuedRequests(This,plcCurrentQueuedRequests) ) 

#define IThreadPoolKnobs_SetMaxQueuedRequests(This,lcMaxQueuedRequests)	\
    ( (This)->lpVtbl -> SetMaxQueuedRequests(This,lcMaxQueuedRequests) ) 

#define IThreadPoolKnobs_SetMinThreads(This,lcMinThreads)	\
    ( (This)->lpVtbl -> SetMinThreads(This,lcMinThreads) ) 

#define IThreadPoolKnobs_SetQueueDepth(This,lcQueueDepth)	\
    ( (This)->lpVtbl -> SetQueueDepth(This,lcQueueDepth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThreadPoolKnobs_INTERFACE_DEFINED__ */


#ifndef __IComStaThreadPoolKnobs_INTERFACE_DEFINED__
#define __IComStaThreadPoolKnobs_INTERFACE_DEFINED__

/* interface IComStaThreadPoolKnobs */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IComStaThreadPoolKnobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("324B64FA-33B6-11d2-98B7-00C04F8EE1C4")
    IComStaThreadPoolKnobs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMinThreadCount( 
            DWORD minThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMinThreadCount( 
            /* [out] */ __RPC__out DWORD *minThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxThreadCount( 
            DWORD maxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxThreadCount( 
            /* [out] */ __RPC__out DWORD *maxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActivityPerThread( 
            DWORD activitiesPerThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityPerThread( 
            /* [out] */ __RPC__out DWORD *activitiesPerThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActivityRatio( 
            DOUBLE activityRatio) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityRatio( 
            /* [out] */ __RPC__out DOUBLE *activityRatio) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThreadCount( 
            /* [out] */ __RPC__out DWORD *pdwThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQueueDepth( 
            /* [out] */ __RPC__out DWORD *pdwQDepth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetQueueDepth( 
            /* [in] */ long dwQDepth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComStaThreadPoolKnobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComStaThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComStaThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetMinThreadCount)
        HRESULT ( STDMETHODCALLTYPE *SetMinThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            DWORD minThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetMinThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetMinThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *minThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *SetMaxThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            DWORD maxThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *maxThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetActivityPerThread)
        HRESULT ( STDMETHODCALLTYPE *SetActivityPerThread )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            DWORD activitiesPerThread);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetActivityPerThread)
        HRESULT ( STDMETHODCALLTYPE *GetActivityPerThread )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *activitiesPerThread);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetActivityRatio)
        HRESULT ( STDMETHODCALLTYPE *SetActivityRatio )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            DOUBLE activityRatio);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetActivityRatio)
        HRESULT ( STDMETHODCALLTYPE *GetActivityRatio )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DOUBLE *activityRatio);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *pdwThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetQueueDepth)
        HRESULT ( STDMETHODCALLTYPE *GetQueueDepth )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *pdwQDepth);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetQueueDepth)
        HRESULT ( STDMETHODCALLTYPE *SetQueueDepth )( 
            __RPC__in IComStaThreadPoolKnobs * This,
            /* [in] */ long dwQDepth);
        
        END_INTERFACE
    } IComStaThreadPoolKnobsVtbl;

    interface IComStaThreadPoolKnobs
    {
        CONST_VTBL struct IComStaThreadPoolKnobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComStaThreadPoolKnobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComStaThreadPoolKnobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComStaThreadPoolKnobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComStaThreadPoolKnobs_SetMinThreadCount(This,minThreads)	\
    ( (This)->lpVtbl -> SetMinThreadCount(This,minThreads) ) 

#define IComStaThreadPoolKnobs_GetMinThreadCount(This,minThreads)	\
    ( (This)->lpVtbl -> GetMinThreadCount(This,minThreads) ) 

#define IComStaThreadPoolKnobs_SetMaxThreadCount(This,maxThreads)	\
    ( (This)->lpVtbl -> SetMaxThreadCount(This,maxThreads) ) 

#define IComStaThreadPoolKnobs_GetMaxThreadCount(This,maxThreads)	\
    ( (This)->lpVtbl -> GetMaxThreadCount(This,maxThreads) ) 

#define IComStaThreadPoolKnobs_SetActivityPerThread(This,activitiesPerThread)	\
    ( (This)->lpVtbl -> SetActivityPerThread(This,activitiesPerThread) ) 

#define IComStaThreadPoolKnobs_GetActivityPerThread(This,activitiesPerThread)	\
    ( (This)->lpVtbl -> GetActivityPerThread(This,activitiesPerThread) ) 

#define IComStaThreadPoolKnobs_SetActivityRatio(This,activityRatio)	\
    ( (This)->lpVtbl -> SetActivityRatio(This,activityRatio) ) 

#define IComStaThreadPoolKnobs_GetActivityRatio(This,activityRatio)	\
    ( (This)->lpVtbl -> GetActivityRatio(This,activityRatio) ) 

#define IComStaThreadPoolKnobs_GetThreadCount(This,pdwThreads)	\
    ( (This)->lpVtbl -> GetThreadCount(This,pdwThreads) ) 

#define IComStaThreadPoolKnobs_GetQueueDepth(This,pdwQDepth)	\
    ( (This)->lpVtbl -> GetQueueDepth(This,pdwQDepth) ) 

#define IComStaThreadPoolKnobs_SetQueueDepth(This,dwQDepth)	\
    ( (This)->lpVtbl -> SetQueueDepth(This,dwQDepth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComStaThreadPoolKnobs_INTERFACE_DEFINED__ */


#ifndef __IComMtaThreadPoolKnobs_INTERFACE_DEFINED__
#define __IComMtaThreadPoolKnobs_INTERFACE_DEFINED__

/* interface IComMtaThreadPoolKnobs */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IComMtaThreadPoolKnobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9A76D2E-76A5-43eb-A0C4-49BEC8E48480")
    IComMtaThreadPoolKnobs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MTASetMaxThreadCount( 
            DWORD dwMaxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MTAGetMaxThreadCount( 
            /* [out] */ __RPC__out DWORD *pdwMaxThreads) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MTASetThrottleValue( 
            DWORD dwThrottle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MTAGetThrottleValue( 
            /* [out] */ __RPC__out DWORD *pdwThrottle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComMtaThreadPoolKnobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComMtaThreadPoolKnobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComMtaThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComMtaThreadPoolKnobs * This);
        
        DECLSPEC_XFGVIRT(IComMtaThreadPoolKnobs, MTASetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *MTASetMaxThreadCount )( 
            __RPC__in IComMtaThreadPoolKnobs * This,
            DWORD dwMaxThreads);
        
        DECLSPEC_XFGVIRT(IComMtaThreadPoolKnobs, MTAGetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *MTAGetMaxThreadCount )( 
            __RPC__in IComMtaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *pdwMaxThreads);
        
        DECLSPEC_XFGVIRT(IComMtaThreadPoolKnobs, MTASetThrottleValue)
        HRESULT ( STDMETHODCALLTYPE *MTASetThrottleValue )( 
            __RPC__in IComMtaThreadPoolKnobs * This,
            DWORD dwThrottle);
        
        DECLSPEC_XFGVIRT(IComMtaThreadPoolKnobs, MTAGetThrottleValue)
        HRESULT ( STDMETHODCALLTYPE *MTAGetThrottleValue )( 
            __RPC__in IComMtaThreadPoolKnobs * This,
            /* [out] */ __RPC__out DWORD *pdwThrottle);
        
        END_INTERFACE
    } IComMtaThreadPoolKnobsVtbl;

    interface IComMtaThreadPoolKnobs
    {
        CONST_VTBL struct IComMtaThreadPoolKnobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComMtaThreadPoolKnobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComMtaThreadPoolKnobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComMtaThreadPoolKnobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComMtaThreadPoolKnobs_MTASetMaxThreadCount(This,dwMaxThreads)	\
    ( (This)->lpVtbl -> MTASetMaxThreadCount(This,dwMaxThreads) ) 

#define IComMtaThreadPoolKnobs_MTAGetMaxThreadCount(This,pdwMaxThreads)	\
    ( (This)->lpVtbl -> MTAGetMaxThreadCount(This,pdwMaxThreads) ) 

#define IComMtaThreadPoolKnobs_MTASetThrottleValue(This,dwThrottle)	\
    ( (This)->lpVtbl -> MTASetThrottleValue(This,dwThrottle) ) 

#define IComMtaThreadPoolKnobs_MTAGetThrottleValue(This,pdwThrottle)	\
    ( (This)->lpVtbl -> MTAGetThrottleValue(This,pdwThrottle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComMtaThreadPoolKnobs_INTERFACE_DEFINED__ */


#ifndef __IComStaThreadPoolKnobs2_INTERFACE_DEFINED__
#define __IComStaThreadPoolKnobs2_INTERFACE_DEFINED__

/* interface IComStaThreadPoolKnobs2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IComStaThreadPoolKnobs2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73707523-FF9A-4974-BF84-2108DC213740")
    IComStaThreadPoolKnobs2 : public IComStaThreadPoolKnobs
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMaxCPULoad( 
            /* [out] */ __RPC__out DWORD *pdwLoad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxCPULoad( 
            long pdwLoad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCPUMetricEnabled( 
            /* [out] */ __RPC__out BOOL *pbMetricEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCPUMetricEnabled( 
            BOOL bMetricEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCreateThreadsAggressively( 
            /* [out] */ __RPC__out BOOL *pbMetricEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCreateThreadsAggressively( 
            BOOL bMetricEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxCSR( 
            /* [out] */ __RPC__out DWORD *pdwCSR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxCSR( 
            long dwCSR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWaitTimeForThreadCleanup( 
            /* [out] */ __RPC__out DWORD *pdwThreadCleanupWaitTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWaitTimeForThreadCleanup( 
            long dwThreadCleanupWaitTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComStaThreadPoolKnobs2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComStaThreadPoolKnobs2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComStaThreadPoolKnobs2 * This);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetMinThreadCount)
        HRESULT ( STDMETHODCALLTYPE *SetMinThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            DWORD minThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetMinThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetMinThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *minThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *SetMaxThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            DWORD maxThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetMaxThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *maxThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetActivityPerThread)
        HRESULT ( STDMETHODCALLTYPE *SetActivityPerThread )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            DWORD activitiesPerThread);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetActivityPerThread)
        HRESULT ( STDMETHODCALLTYPE *GetActivityPerThread )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *activitiesPerThread);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetActivityRatio)
        HRESULT ( STDMETHODCALLTYPE *SetActivityRatio )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            DOUBLE activityRatio);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetActivityRatio)
        HRESULT ( STDMETHODCALLTYPE *GetActivityRatio )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DOUBLE *activityRatio);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetThreadCount)
        HRESULT ( STDMETHODCALLTYPE *GetThreadCount )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *pdwThreads);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, GetQueueDepth)
        HRESULT ( STDMETHODCALLTYPE *GetQueueDepth )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *pdwQDepth);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs, SetQueueDepth)
        HRESULT ( STDMETHODCALLTYPE *SetQueueDepth )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [in] */ long dwQDepth);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, GetMaxCPULoad)
        HRESULT ( STDMETHODCALLTYPE *GetMaxCPULoad )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *pdwLoad);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, SetMaxCPULoad)
        HRESULT ( STDMETHODCALLTYPE *SetMaxCPULoad )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            long pdwLoad);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, GetCPUMetricEnabled)
        HRESULT ( STDMETHODCALLTYPE *GetCPUMetricEnabled )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out BOOL *pbMetricEnabled);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, SetCPUMetricEnabled)
        HRESULT ( STDMETHODCALLTYPE *SetCPUMetricEnabled )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            BOOL bMetricEnabled);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, GetCreateThreadsAggressively)
        HRESULT ( STDMETHODCALLTYPE *GetCreateThreadsAggressively )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out BOOL *pbMetricEnabled);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, SetCreateThreadsAggressively)
        HRESULT ( STDMETHODCALLTYPE *SetCreateThreadsAggressively )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            BOOL bMetricEnabled);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, GetMaxCSR)
        HRESULT ( STDMETHODCALLTYPE *GetMaxCSR )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *pdwCSR);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, SetMaxCSR)
        HRESULT ( STDMETHODCALLTYPE *SetMaxCSR )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            long dwCSR);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, GetWaitTimeForThreadCleanup)
        HRESULT ( STDMETHODCALLTYPE *GetWaitTimeForThreadCleanup )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            /* [out] */ __RPC__out DWORD *pdwThreadCleanupWaitTime);
        
        DECLSPEC_XFGVIRT(IComStaThreadPoolKnobs2, SetWaitTimeForThreadCleanup)
        HRESULT ( STDMETHODCALLTYPE *SetWaitTimeForThreadCleanup )( 
            __RPC__in IComStaThreadPoolKnobs2 * This,
            long dwThreadCleanupWaitTime);
        
        END_INTERFACE
    } IComStaThreadPoolKnobs2Vtbl;

    interface IComStaThreadPoolKnobs2
    {
        CONST_VTBL struct IComStaThreadPoolKnobs2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComStaThreadPoolKnobs2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComStaThreadPoolKnobs2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComStaThreadPoolKnobs2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComStaThreadPoolKnobs2_SetMinThreadCount(This,minThreads)	\
    ( (This)->lpVtbl -> SetMinThreadCount(This,minThreads) ) 

#define IComStaThreadPoolKnobs2_GetMinThreadCount(This,minThreads)	\
    ( (This)->lpVtbl -> GetMinThreadCount(This,minThreads) ) 

#define IComStaThreadPoolKnobs2_SetMaxThreadCount(This,maxThreads)	\
    ( (This)->lpVtbl -> SetMaxThreadCount(This,maxThreads) ) 

#define IComStaThreadPoolKnobs2_GetMaxThreadCount(This,maxThreads)	\
    ( (This)->lpVtbl -> GetMaxThreadCount(This,maxThreads) ) 

#define IComStaThreadPoolKnobs2_SetActivityPerThread(This,activitiesPerThread)	\
    ( (This)->lpVtbl -> SetActivityPerThread(This,activitiesPerThread) ) 

#define IComStaThreadPoolKnobs2_GetActivityPerThread(This,activitiesPerThread)	\
    ( (This)->lpVtbl -> GetActivityPerThread(This,activitiesPerThread) ) 

#define IComStaThreadPoolKnobs2_SetActivityRatio(This,activityRatio)	\
    ( (This)->lpVtbl -> SetActivityRatio(This,activityRatio) ) 

#define IComStaThreadPoolKnobs2_GetActivityRatio(This,activityRatio)	\
    ( (This)->lpVtbl -> GetActivityRatio(This,activityRatio) ) 

#define IComStaThreadPoolKnobs2_GetThreadCount(This,pdwThreads)	\
    ( (This)->lpVtbl -> GetThreadCount(This,pdwThreads) ) 

#define IComStaThreadPoolKnobs2_GetQueueDepth(This,pdwQDepth)	\
    ( (This)->lpVtbl -> GetQueueDepth(This,pdwQDepth) ) 

#define IComStaThreadPoolKnobs2_SetQueueDepth(This,dwQDepth)	\
    ( (This)->lpVtbl -> SetQueueDepth(This,dwQDepth) ) 


#define IComStaThreadPoolKnobs2_GetMaxCPULoad(This,pdwLoad)	\
    ( (This)->lpVtbl -> GetMaxCPULoad(This,pdwLoad) ) 

#define IComStaThreadPoolKnobs2_SetMaxCPULoad(This,pdwLoad)	\
    ( (This)->lpVtbl -> SetMaxCPULoad(This,pdwLoad) ) 

#define IComStaThreadPoolKnobs2_GetCPUMetricEnabled(This,pbMetricEnabled)	\
    ( (This)->lpVtbl -> GetCPUMetricEnabled(This,pbMetricEnabled) ) 

#define IComStaThreadPoolKnobs2_SetCPUMetricEnabled(This,bMetricEnabled)	\
    ( (This)->lpVtbl -> SetCPUMetricEnabled(This,bMetricEnabled) ) 

#define IComStaThreadPoolKnobs2_GetCreateThreadsAggressively(This,pbMetricEnabled)	\
    ( (This)->lpVtbl -> GetCreateThreadsAggressively(This,pbMetricEnabled) ) 

#define IComStaThreadPoolKnobs2_SetCreateThreadsAggressively(This,bMetricEnabled)	\
    ( (This)->lpVtbl -> SetCreateThreadsAggressively(This,bMetricEnabled) ) 

#define IComStaThreadPoolKnobs2_GetMaxCSR(This,pdwCSR)	\
    ( (This)->lpVtbl -> GetMaxCSR(This,pdwCSR) ) 

#define IComStaThreadPoolKnobs2_SetMaxCSR(This,dwCSR)	\
    ( (This)->lpVtbl -> SetMaxCSR(This,dwCSR) ) 

#define IComStaThreadPoolKnobs2_GetWaitTimeForThreadCleanup(This,pdwThreadCleanupWaitTime)	\
    ( (This)->lpVtbl -> GetWaitTimeForThreadCleanup(This,pdwThreadCleanupWaitTime) ) 

#define IComStaThreadPoolKnobs2_SetWaitTimeForThreadCleanup(This,dwThreadCleanupWaitTime)	\
    ( (This)->lpVtbl -> SetWaitTimeForThreadCleanup(This,dwThreadCleanupWaitTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComStaThreadPoolKnobs2_INTERFACE_DEFINED__ */


#ifndef __IProcessInitializer_INTERFACE_DEFINED__
#define __IProcessInitializer_INTERFACE_DEFINED__

/* interface IProcessInitializer */
/* [oleautomation][uuid][unique][object] */ 


EXTERN_C const IID IID_IProcessInitializer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1113f52d-dc7f-4943-aed6-88d04027e32a")
    IProcessInitializer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Startup( 
            /* [in] */ __RPC__in_opt IUnknown *punkProcessControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProcessInitializerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProcessInitializer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProcessInitializer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProcessInitializer * This);
        
        DECLSPEC_XFGVIRT(IProcessInitializer, Startup)
        HRESULT ( STDMETHODCALLTYPE *Startup )( 
            __RPC__in IProcessInitializer * This,
            /* [in] */ __RPC__in_opt IUnknown *punkProcessControl);
        
        DECLSPEC_XFGVIRT(IProcessInitializer, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IProcessInitializer * This);
        
        END_INTERFACE
    } IProcessInitializerVtbl;

    interface IProcessInitializer
    {
        CONST_VTBL struct IProcessInitializerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProcessInitializer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProcessInitializer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProcessInitializer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProcessInitializer_Startup(This,punkProcessControl)	\
    ( (This)->lpVtbl -> Startup(This,punkProcessControl) ) 

#define IProcessInitializer_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProcessInitializer_INTERFACE_DEFINED__ */


#ifndef __IServicePoolConfig_INTERFACE_DEFINED__
#define __IServicePoolConfig_INTERFACE_DEFINED__

/* interface IServicePoolConfig */
/* [object][uuid][unique][local] */ 


EXTERN_C const IID IID_IServicePoolConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a9690656-5bca-470c-8451-250c1f43a33e")
    IServicePoolConfig : public IUnknown
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaxPoolSize( 
            /* [in] */ DWORD dwMaxPool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaxPoolSize( 
            /* [retval][out] */ DWORD *pdwMaxPool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MinPoolSize( 
            /* [in] */ DWORD dwMinPool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MinPoolSize( 
            /* [retval][out] */ DWORD *pdwMinPool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CreationTimeout( 
            /* [in] */ DWORD dwCreationTimeout) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CreationTimeout( 
            /* [retval][out] */ DWORD *pdwCreationTimeout) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TransactionAffinity( 
            /* [in] */ BOOL fTxAffinity) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TransactionAffinity( 
            /* [retval][out] */ BOOL *pfTxAffinity) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClassFactory( 
            /* [in] */ IClassFactory *pFactory) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClassFactory( 
            /* [retval][out] */ IClassFactory **pFactory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServicePoolConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IServicePoolConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IServicePoolConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IServicePoolConfig * This);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, put_MaxPoolSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxPoolSize )( 
            IServicePoolConfig * This,
            /* [in] */ DWORD dwMaxPool);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, get_MaxPoolSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxPoolSize )( 
            IServicePoolConfig * This,
            /* [retval][out] */ DWORD *pdwMaxPool);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, put_MinPoolSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinPoolSize )( 
            IServicePoolConfig * This,
            /* [in] */ DWORD dwMinPool);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, get_MinPoolSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinPoolSize )( 
            IServicePoolConfig * This,
            /* [retval][out] */ DWORD *pdwMinPool);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, put_CreationTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTimeout )( 
            IServicePoolConfig * This,
            /* [in] */ DWORD dwCreationTimeout);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, get_CreationTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTimeout )( 
            IServicePoolConfig * This,
            /* [retval][out] */ DWORD *pdwCreationTimeout);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, put_TransactionAffinity)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TransactionAffinity )( 
            IServicePoolConfig * This,
            /* [in] */ BOOL fTxAffinity);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, get_TransactionAffinity)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionAffinity )( 
            IServicePoolConfig * This,
            /* [retval][out] */ BOOL *pfTxAffinity);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, put_ClassFactory)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClassFactory )( 
            IServicePoolConfig * This,
            /* [in] */ IClassFactory *pFactory);
        
        DECLSPEC_XFGVIRT(IServicePoolConfig, get_ClassFactory)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClassFactory )( 
            IServicePoolConfig * This,
            /* [retval][out] */ IClassFactory **pFactory);
        
        END_INTERFACE
    } IServicePoolConfigVtbl;

    interface IServicePoolConfig
    {
        CONST_VTBL struct IServicePoolConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServicePoolConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServicePoolConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServicePoolConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServicePoolConfig_put_MaxPoolSize(This,dwMaxPool)	\
    ( (This)->lpVtbl -> put_MaxPoolSize(This,dwMaxPool) ) 

#define IServicePoolConfig_get_MaxPoolSize(This,pdwMaxPool)	\
    ( (This)->lpVtbl -> get_MaxPoolSize(This,pdwMaxPool) ) 

#define IServicePoolConfig_put_MinPoolSize(This,dwMinPool)	\
    ( (This)->lpVtbl -> put_MinPoolSize(This,dwMinPool) ) 

#define IServicePoolConfig_get_MinPoolSize(This,pdwMinPool)	\
    ( (This)->lpVtbl -> get_MinPoolSize(This,pdwMinPool) ) 

#define IServicePoolConfig_put_CreationTimeout(This,dwCreationTimeout)	\
    ( (This)->lpVtbl -> put_CreationTimeout(This,dwCreationTimeout) ) 

#define IServicePoolConfig_get_CreationTimeout(This,pdwCreationTimeout)	\
    ( (This)->lpVtbl -> get_CreationTimeout(This,pdwCreationTimeout) ) 

#define IServicePoolConfig_put_TransactionAffinity(This,fTxAffinity)	\
    ( (This)->lpVtbl -> put_TransactionAffinity(This,fTxAffinity) ) 

#define IServicePoolConfig_get_TransactionAffinity(This,pfTxAffinity)	\
    ( (This)->lpVtbl -> get_TransactionAffinity(This,pfTxAffinity) ) 

#define IServicePoolConfig_put_ClassFactory(This,pFactory)	\
    ( (This)->lpVtbl -> put_ClassFactory(This,pFactory) ) 

#define IServicePoolConfig_get_ClassFactory(This,pFactory)	\
    ( (This)->lpVtbl -> get_ClassFactory(This,pFactory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServicePoolConfig_INTERFACE_DEFINED__ */


#ifndef __IServicePool_INTERFACE_DEFINED__
#define __IServicePool_INTERFACE_DEFINED__

/* interface IServicePool */
/* [object][uuid][unique][local] */ 


EXTERN_C const IID IID_IServicePool;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b302df81-ea45-451e-99a2-09f9fd1b1e13")
    IServicePool : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IUnknown *pPoolConfig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServicePoolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IServicePool * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IServicePool * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IServicePool * This);
        
        DECLSPEC_XFGVIRT(IServicePool, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IServicePool * This,
            /* [in] */ IUnknown *pPoolConfig);
        
        DECLSPEC_XFGVIRT(IServicePool, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            IServicePool * This,
            REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IServicePool, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IServicePool * This);
        
        END_INTERFACE
    } IServicePoolVtbl;

    interface IServicePool
    {
        CONST_VTBL struct IServicePoolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServicePool_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServicePool_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServicePool_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServicePool_Initialize(This,pPoolConfig)	\
    ( (This)->lpVtbl -> Initialize(This,pPoolConfig) ) 

#define IServicePool_GetObject(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetObject(This,riid,ppv) ) 

#define IServicePool_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServicePool_INTERFACE_DEFINED__ */


#ifndef __IManagedPooledObj_INTERFACE_DEFINED__
#define __IManagedPooledObj_INTERFACE_DEFINED__

/* interface IManagedPooledObj */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IManagedPooledObj;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c5da4bea-1b42-4437-8926-b6a38860a770")
    IManagedPooledObj : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetHeld( 
            /* [in] */ BOOL m_bHeld) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IManagedPooledObjVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IManagedPooledObj * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IManagedPooledObj * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IManagedPooledObj * This);
        
        DECLSPEC_XFGVIRT(IManagedPooledObj, SetHeld)
        HRESULT ( STDMETHODCALLTYPE *SetHeld )( 
            IManagedPooledObj * This,
            /* [in] */ BOOL m_bHeld);
        
        END_INTERFACE
    } IManagedPooledObjVtbl;

    interface IManagedPooledObj
    {
        CONST_VTBL struct IManagedPooledObjVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IManagedPooledObj_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IManagedPooledObj_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IManagedPooledObj_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IManagedPooledObj_SetHeld(This,m_bHeld)	\
    ( (This)->lpVtbl -> SetHeld(This,m_bHeld) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IManagedPooledObj_INTERFACE_DEFINED__ */


#ifndef __IManagedPoolAction_INTERFACE_DEFINED__
#define __IManagedPoolAction_INTERFACE_DEFINED__

/* interface IManagedPoolAction */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IManagedPoolAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("da91b74e-5388-4783-949d-c1cd5fb00506")
    IManagedPoolAction : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LastRelease( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IManagedPoolActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IManagedPoolAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IManagedPoolAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IManagedPoolAction * This);
        
        DECLSPEC_XFGVIRT(IManagedPoolAction, LastRelease)
        HRESULT ( STDMETHODCALLTYPE *LastRelease )( 
            IManagedPoolAction * This);
        
        END_INTERFACE
    } IManagedPoolActionVtbl;

    interface IManagedPoolAction
    {
        CONST_VTBL struct IManagedPoolActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IManagedPoolAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IManagedPoolAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IManagedPoolAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IManagedPoolAction_LastRelease(This)	\
    ( (This)->lpVtbl -> LastRelease(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IManagedPoolAction_INTERFACE_DEFINED__ */


#ifndef __IManagedObjectInfo_INTERFACE_DEFINED__
#define __IManagedObjectInfo_INTERFACE_DEFINED__

/* interface IManagedObjectInfo */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IManagedObjectInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1427c51a-4584-49d8-90a0-c50d8086cbe9")
    IManagedObjectInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIUnknown( 
            /* [out] */ IUnknown **pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIObjectControl( 
            /* [out] */ IObjectControl **pCtrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInPool( 
            /* [in] */ BOOL bInPool,
            /* [in] */ IManagedPooledObj *pPooledObj) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWrapperStrength( 
            /* [in] */ BOOL bStrong) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IManagedObjectInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IManagedObjectInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IManagedObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IManagedObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IManagedObjectInfo, GetIUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetIUnknown )( 
            IManagedObjectInfo * This,
            /* [out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(IManagedObjectInfo, GetIObjectControl)
        HRESULT ( STDMETHODCALLTYPE *GetIObjectControl )( 
            IManagedObjectInfo * This,
            /* [out] */ IObjectControl **pCtrl);
        
        DECLSPEC_XFGVIRT(IManagedObjectInfo, SetInPool)
        HRESULT ( STDMETHODCALLTYPE *SetInPool )( 
            IManagedObjectInfo * This,
            /* [in] */ BOOL bInPool,
            /* [in] */ IManagedPooledObj *pPooledObj);
        
        DECLSPEC_XFGVIRT(IManagedObjectInfo, SetWrapperStrength)
        HRESULT ( STDMETHODCALLTYPE *SetWrapperStrength )( 
            IManagedObjectInfo * This,
            /* [in] */ BOOL bStrong);
        
        END_INTERFACE
    } IManagedObjectInfoVtbl;

    interface IManagedObjectInfo
    {
        CONST_VTBL struct IManagedObjectInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IManagedObjectInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IManagedObjectInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IManagedObjectInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IManagedObjectInfo_GetIUnknown(This,pUnk)	\
    ( (This)->lpVtbl -> GetIUnknown(This,pUnk) ) 

#define IManagedObjectInfo_GetIObjectControl(This,pCtrl)	\
    ( (This)->lpVtbl -> GetIObjectControl(This,pCtrl) ) 

#define IManagedObjectInfo_SetInPool(This,bInPool,pPooledObj)	\
    ( (This)->lpVtbl -> SetInPool(This,bInPool,pPooledObj) ) 

#define IManagedObjectInfo_SetWrapperStrength(This,bStrong)	\
    ( (This)->lpVtbl -> SetWrapperStrength(This,bStrong) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IManagedObjectInfo_INTERFACE_DEFINED__ */


#ifndef __IAppDomainHelper_INTERFACE_DEFINED__
#define __IAppDomainHelper_INTERFACE_DEFINED__

/* interface IAppDomainHelper */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAppDomainHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7b67079-8255-42c6-9ec0-6994a3548780")
    IAppDomainHelper : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IUnknown *pUnkAD,
            /* [in] */ HRESULT ( STDMETHODCALLTYPE __MIDL__IAppDomainHelper0000 )( 
                void *pv),
            /* [in] */ void *pPool) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoCallback( 
            /* [in] */ IUnknown *pUnkAD,
            /* [in] */ HRESULT ( STDMETHODCALLTYPE __MIDL__IAppDomainHelper0001 )( 
                void *pv),
            /* [in] */ void *pPool) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppDomainHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppDomainHelper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppDomainHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppDomainHelper * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IAppDomainHelper * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IAppDomainHelper * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IAppDomainHelper * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAppDomainHelper * This,
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
        
        DECLSPEC_XFGVIRT(IAppDomainHelper, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IAppDomainHelper * This,
            /* [in] */ IUnknown *pUnkAD,
            /* [in] */ HRESULT ( STDMETHODCALLTYPE __MIDL__IAppDomainHelper0000 )( 
                void *pv),
            /* [in] */ void *pPool);
        
        DECLSPEC_XFGVIRT(IAppDomainHelper, DoCallback)
        HRESULT ( STDMETHODCALLTYPE *DoCallback )( 
            IAppDomainHelper * This,
            /* [in] */ IUnknown *pUnkAD,
            /* [in] */ HRESULT ( STDMETHODCALLTYPE __MIDL__IAppDomainHelper0001 )( 
                void *pv),
            /* [in] */ void *pPool);
        
        END_INTERFACE
    } IAppDomainHelperVtbl;

    interface IAppDomainHelper
    {
        CONST_VTBL struct IAppDomainHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppDomainHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppDomainHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppDomainHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppDomainHelper_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAppDomainHelper_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAppDomainHelper_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAppDomainHelper_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAppDomainHelper_Initialize(This,pUnkAD,__MIDL__IAppDomainHelper0000,pPool)	\
    ( (This)->lpVtbl -> Initialize(This,pUnkAD,__MIDL__IAppDomainHelper0000,pPool) ) 

#define IAppDomainHelper_DoCallback(This,pUnkAD,__MIDL__IAppDomainHelper0001,pPool)	\
    ( (This)->lpVtbl -> DoCallback(This,pUnkAD,__MIDL__IAppDomainHelper0001,pPool) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppDomainHelper_INTERFACE_DEFINED__ */


#ifndef __IAssemblyLocator_INTERFACE_DEFINED__
#define __IAssemblyLocator_INTERFACE_DEFINED__

/* interface IAssemblyLocator */
/* [object][oleautomation][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IAssemblyLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("391ffbb9-a8ee-432a-abc8-baa238dab90f")
    IAssemblyLocator : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetModules( 
            /* [in] */ __RPC__in BSTR applicationDir,
            /* [in] */ __RPC__in BSTR applicationName,
            /* [in] */ __RPC__in BSTR assemblyName,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pModules) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAssemblyLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAssemblyLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAssemblyLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAssemblyLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAssemblyLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAssemblyLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAssemblyLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAssemblyLocator * This,
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
        
        DECLSPEC_XFGVIRT(IAssemblyLocator, GetModules)
        HRESULT ( STDMETHODCALLTYPE *GetModules )( 
            __RPC__in IAssemblyLocator * This,
            /* [in] */ __RPC__in BSTR applicationDir,
            /* [in] */ __RPC__in BSTR applicationName,
            /* [in] */ __RPC__in BSTR assemblyName,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pModules);
        
        END_INTERFACE
    } IAssemblyLocatorVtbl;

    interface IAssemblyLocator
    {
        CONST_VTBL struct IAssemblyLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAssemblyLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAssemblyLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAssemblyLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAssemblyLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAssemblyLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAssemblyLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAssemblyLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAssemblyLocator_GetModules(This,applicationDir,applicationName,assemblyName,pModules)	\
    ( (This)->lpVtbl -> GetModules(This,applicationDir,applicationName,assemblyName,pModules) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAssemblyLocator_INTERFACE_DEFINED__ */


#ifndef __IManagedActivationEvents_INTERFACE_DEFINED__
#define __IManagedActivationEvents_INTERFACE_DEFINED__

/* interface IManagedActivationEvents */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IManagedActivationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a5f325af-572f-46da-b8ab-827c3d95d99e")
    IManagedActivationEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateManagedStub( 
            /* [in] */ IManagedObjectInfo *pInfo,
            /* [in] */ BOOL fDist) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyManagedStub( 
            /* [in] */ IManagedObjectInfo *pInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IManagedActivationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IManagedActivationEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IManagedActivationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IManagedActivationEvents * This);
        
        DECLSPEC_XFGVIRT(IManagedActivationEvents, CreateManagedStub)
        HRESULT ( STDMETHODCALLTYPE *CreateManagedStub )( 
            IManagedActivationEvents * This,
            /* [in] */ IManagedObjectInfo *pInfo,
            /* [in] */ BOOL fDist);
        
        DECLSPEC_XFGVIRT(IManagedActivationEvents, DestroyManagedStub)
        HRESULT ( STDMETHODCALLTYPE *DestroyManagedStub )( 
            IManagedActivationEvents * This,
            /* [in] */ IManagedObjectInfo *pInfo);
        
        END_INTERFACE
    } IManagedActivationEventsVtbl;

    interface IManagedActivationEvents
    {
        CONST_VTBL struct IManagedActivationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IManagedActivationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IManagedActivationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IManagedActivationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IManagedActivationEvents_CreateManagedStub(This,pInfo,fDist)	\
    ( (This)->lpVtbl -> CreateManagedStub(This,pInfo,fDist) ) 

#define IManagedActivationEvents_DestroyManagedStub(This,pInfo)	\
    ( (This)->lpVtbl -> DestroyManagedStub(This,pInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IManagedActivationEvents_INTERFACE_DEFINED__ */


#ifndef __ISendMethodEvents_INTERFACE_DEFINED__
#define __ISendMethodEvents_INTERFACE_DEFINED__

/* interface ISendMethodEvents */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_ISendMethodEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2732fd59-b2b4-4d44-878c-8b8f09626008")
    ISendMethodEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SendMethodCall( 
            /* [in] */ const void *pIdentity,
            /* [in] */ REFIID riid,
            /* [in] */ DWORD dwMeth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendMethodReturn( 
            /* [in] */ const void *pIdentity,
            /* [in] */ REFIID riid,
            /* [in] */ DWORD dwMeth,
            /* [in] */ HRESULT hrCall,
            /* [in] */ HRESULT hrServer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISendMethodEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISendMethodEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISendMethodEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISendMethodEvents * This);
        
        DECLSPEC_XFGVIRT(ISendMethodEvents, SendMethodCall)
        HRESULT ( STDMETHODCALLTYPE *SendMethodCall )( 
            ISendMethodEvents * This,
            /* [in] */ const void *pIdentity,
            /* [in] */ REFIID riid,
            /* [in] */ DWORD dwMeth);
        
        DECLSPEC_XFGVIRT(ISendMethodEvents, SendMethodReturn)
        HRESULT ( STDMETHODCALLTYPE *SendMethodReturn )( 
            ISendMethodEvents * This,
            /* [in] */ const void *pIdentity,
            /* [in] */ REFIID riid,
            /* [in] */ DWORD dwMeth,
            /* [in] */ HRESULT hrCall,
            /* [in] */ HRESULT hrServer);
        
        END_INTERFACE
    } ISendMethodEventsVtbl;

    interface ISendMethodEvents
    {
        CONST_VTBL struct ISendMethodEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISendMethodEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISendMethodEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISendMethodEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISendMethodEvents_SendMethodCall(This,pIdentity,riid,dwMeth)	\
    ( (This)->lpVtbl -> SendMethodCall(This,pIdentity,riid,dwMeth) ) 

#define ISendMethodEvents_SendMethodReturn(This,pIdentity,riid,dwMeth,hrCall,hrServer)	\
    ( (This)->lpVtbl -> SendMethodReturn(This,pIdentity,riid,dwMeth,hrCall,hrServer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISendMethodEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0113 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0113_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0113_v0_0_s_ifspec;

#ifndef __ITransactionResourcePool_INTERFACE_DEFINED__
#define __ITransactionResourcePool_INTERFACE_DEFINED__

/* interface ITransactionResourcePool */
/* [object][unique][local][helpstring][uuid] */ 


EXTERN_C const IID IID_ITransactionResourcePool;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C5FEB7C1-346A-11D1-B1CC-00AA00BA3258")
    ITransactionResourcePool : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutResource( 
            /* [in] */ IObjPool *pPool,
            /* [in] */ IUnknown *pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResource( 
            /* [in] */ IObjPool *pPool,
            /* [out] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionResourcePoolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITransactionResourcePool * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITransactionResourcePool * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITransactionResourcePool * This);
        
        DECLSPEC_XFGVIRT(ITransactionResourcePool, PutResource)
        HRESULT ( STDMETHODCALLTYPE *PutResource )( 
            ITransactionResourcePool * This,
            /* [in] */ IObjPool *pPool,
            /* [in] */ IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(ITransactionResourcePool, GetResource)
        HRESULT ( STDMETHODCALLTYPE *GetResource )( 
            ITransactionResourcePool * This,
            /* [in] */ IObjPool *pPool,
            /* [out] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } ITransactionResourcePoolVtbl;

    interface ITransactionResourcePool
    {
        CONST_VTBL struct ITransactionResourcePoolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionResourcePool_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionResourcePool_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionResourcePool_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionResourcePool_PutResource(This,pPool,pUnk)	\
    ( (This)->lpVtbl -> PutResource(This,pPool,pUnk) ) 

#define ITransactionResourcePool_GetResource(This,pPool,ppUnk)	\
    ( (This)->lpVtbl -> GetResource(This,pPool,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionResourcePool_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_autosvcs_0000_0114 */
/* [local] */ 

EXTERN_C HRESULT __stdcall MTSCreateActivity ( REFIID riid, void** ppobj );


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0114_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0114_v0_0_s_ifspec;

#ifndef __IMTSCall_INTERFACE_DEFINED__
#define __IMTSCall_INTERFACE_DEFINED__

/* interface IMTSCall */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IMTSCall;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372AEF-CAE7-11CF-BE81-00AA00A2FA25")
    IMTSCall : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCall( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMTSCallVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMTSCall * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMTSCall * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMTSCall * This);
        
        DECLSPEC_XFGVIRT(IMTSCall, OnCall)
        HRESULT ( STDMETHODCALLTYPE *OnCall )( 
            IMTSCall * This);
        
        END_INTERFACE
    } IMTSCallVtbl;

    interface IMTSCall
    {
        CONST_VTBL struct IMTSCallVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMTSCall_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMTSCall_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMTSCall_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMTSCall_OnCall(This)	\
    ( (This)->lpVtbl -> OnCall(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMTSCall_INTERFACE_DEFINED__ */


#ifndef __IContextProperties_INTERFACE_DEFINED__
#define __IContextProperties_INTERFACE_DEFINED__

/* interface IContextProperties */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IContextProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D396DA85-BF8F-11d1-BBAE-00C04FC2FA5F")
    IContextProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [retval][out] */ long *plCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *pProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumNames( 
            /* [retval][out] */ IEnumNames **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ BSTR name,
            /* [in] */ VARIANT property) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveProperty( 
            /* [in] */ BSTR name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IContextProperties * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IContextProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IContextProperties * This);
        
        DECLSPEC_XFGVIRT(IContextProperties, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            IContextProperties * This,
            /* [retval][out] */ long *plCount);
        
        DECLSPEC_XFGVIRT(IContextProperties, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IContextProperties * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(IContextProperties, EnumNames)
        HRESULT ( STDMETHODCALLTYPE *EnumNames )( 
            IContextProperties * This,
            /* [retval][out] */ IEnumNames **ppenum);
        
        DECLSPEC_XFGVIRT(IContextProperties, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IContextProperties * This,
            /* [in] */ BSTR name,
            /* [in] */ VARIANT property);
        
        DECLSPEC_XFGVIRT(IContextProperties, RemoveProperty)
        HRESULT ( STDMETHODCALLTYPE *RemoveProperty )( 
            IContextProperties * This,
            /* [in] */ BSTR name);
        
        END_INTERFACE
    } IContextPropertiesVtbl;

    interface IContextProperties
    {
        CONST_VTBL struct IContextPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextProperties_Count(This,plCount)	\
    ( (This)->lpVtbl -> Count(This,plCount) ) 

#define IContextProperties_GetProperty(This,name,pProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,name,pProperty) ) 

#define IContextProperties_EnumNames(This,ppenum)	\
    ( (This)->lpVtbl -> EnumNames(This,ppenum) ) 

#define IContextProperties_SetProperty(This,name,property)	\
    ( (This)->lpVtbl -> SetProperty(This,name,property) ) 

#define IContextProperties_RemoveProperty(This,name)	\
    ( (This)->lpVtbl -> RemoveProperty(This,name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextProperties_INTERFACE_DEFINED__ */


#ifndef __IObjPool_INTERFACE_DEFINED__
#define __IObjPool_INTERFACE_DEFINED__

/* interface IObjPool */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IObjPool;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7D8805A0-2EA7-11D1-B1CC-00AA00BA3258")
    IObjPool : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE Reserved1( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved2( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved3( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved4( void) = 0;
        
        virtual void STDMETHODCALLTYPE PutEndTx( 
            IUnknown *pObj) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved5( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved6( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjPoolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjPool * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved1)
        void ( STDMETHODCALLTYPE *Reserved1 )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved2)
        void ( STDMETHODCALLTYPE *Reserved2 )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved3)
        void ( STDMETHODCALLTYPE *Reserved3 )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved4)
        void ( STDMETHODCALLTYPE *Reserved4 )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, PutEndTx)
        void ( STDMETHODCALLTYPE *PutEndTx )( 
            IObjPool * This,
            IUnknown *pObj);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved5)
        void ( STDMETHODCALLTYPE *Reserved5 )( 
            IObjPool * This);
        
        DECLSPEC_XFGVIRT(IObjPool, Reserved6)
        void ( STDMETHODCALLTYPE *Reserved6 )( 
            IObjPool * This);
        
        END_INTERFACE
    } IObjPoolVtbl;

    interface IObjPool
    {
        CONST_VTBL struct IObjPoolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjPool_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjPool_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjPool_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjPool_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define IObjPool_Reserved2(This)	\
    ( (This)->lpVtbl -> Reserved2(This) ) 

#define IObjPool_Reserved3(This)	\
    ( (This)->lpVtbl -> Reserved3(This) ) 

#define IObjPool_Reserved4(This)	\
    ( (This)->lpVtbl -> Reserved4(This) ) 

#define IObjPool_PutEndTx(This,pObj)	\
    ( (This)->lpVtbl -> PutEndTx(This,pObj) ) 

#define IObjPool_Reserved5(This)	\
    ( (This)->lpVtbl -> Reserved5(This) ) 

#define IObjPool_Reserved6(This)	\
    ( (This)->lpVtbl -> Reserved6(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjPool_INTERFACE_DEFINED__ */


#ifndef __ITransactionProperty_INTERFACE_DEFINED__
#define __ITransactionProperty_INTERFACE_DEFINED__

/* interface ITransactionProperty */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_ITransactionProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("788ea814-87b1-11d1-bba6-00c04fc2fa5f")
    ITransactionProperty : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE Reserved1( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved2( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved3( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved4( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved5( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved6( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved7( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved8( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved9( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransactionResourcePool( 
            /* [out] */ ITransactionResourcePool **ppTxPool) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved10( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved11( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved12( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved13( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved14( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved15( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved16( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved17( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITransactionProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved1)
        void ( STDMETHODCALLTYPE *Reserved1 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved2)
        void ( STDMETHODCALLTYPE *Reserved2 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved3)
        void ( STDMETHODCALLTYPE *Reserved3 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved4)
        void ( STDMETHODCALLTYPE *Reserved4 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved5)
        void ( STDMETHODCALLTYPE *Reserved5 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved6)
        void ( STDMETHODCALLTYPE *Reserved6 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved7)
        void ( STDMETHODCALLTYPE *Reserved7 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved8)
        void ( STDMETHODCALLTYPE *Reserved8 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved9)
        void ( STDMETHODCALLTYPE *Reserved9 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, GetTransactionResourcePool)
        HRESULT ( STDMETHODCALLTYPE *GetTransactionResourcePool )( 
            ITransactionProperty * This,
            /* [out] */ ITransactionResourcePool **ppTxPool);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved10)
        void ( STDMETHODCALLTYPE *Reserved10 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved11)
        void ( STDMETHODCALLTYPE *Reserved11 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved12)
        void ( STDMETHODCALLTYPE *Reserved12 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved13)
        void ( STDMETHODCALLTYPE *Reserved13 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved14)
        void ( STDMETHODCALLTYPE *Reserved14 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved15)
        void ( STDMETHODCALLTYPE *Reserved15 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved16)
        void ( STDMETHODCALLTYPE *Reserved16 )( 
            ITransactionProperty * This);
        
        DECLSPEC_XFGVIRT(ITransactionProperty, Reserved17)
        void ( STDMETHODCALLTYPE *Reserved17 )( 
            ITransactionProperty * This);
        
        END_INTERFACE
    } ITransactionPropertyVtbl;

    interface ITransactionProperty
    {
        CONST_VTBL struct ITransactionPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionProperty_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define ITransactionProperty_Reserved2(This)	\
    ( (This)->lpVtbl -> Reserved2(This) ) 

#define ITransactionProperty_Reserved3(This)	\
    ( (This)->lpVtbl -> Reserved3(This) ) 

#define ITransactionProperty_Reserved4(This)	\
    ( (This)->lpVtbl -> Reserved4(This) ) 

#define ITransactionProperty_Reserved5(This)	\
    ( (This)->lpVtbl -> Reserved5(This) ) 

#define ITransactionProperty_Reserved6(This)	\
    ( (This)->lpVtbl -> Reserved6(This) ) 

#define ITransactionProperty_Reserved7(This)	\
    ( (This)->lpVtbl -> Reserved7(This) ) 

#define ITransactionProperty_Reserved8(This)	\
    ( (This)->lpVtbl -> Reserved8(This) ) 

#define ITransactionProperty_Reserved9(This)	\
    ( (This)->lpVtbl -> Reserved9(This) ) 

#define ITransactionProperty_GetTransactionResourcePool(This,ppTxPool)	\
    ( (This)->lpVtbl -> GetTransactionResourcePool(This,ppTxPool) ) 

#define ITransactionProperty_Reserved10(This)	\
    ( (This)->lpVtbl -> Reserved10(This) ) 

#define ITransactionProperty_Reserved11(This)	\
    ( (This)->lpVtbl -> Reserved11(This) ) 

#define ITransactionProperty_Reserved12(This)	\
    ( (This)->lpVtbl -> Reserved12(This) ) 

#define ITransactionProperty_Reserved13(This)	\
    ( (This)->lpVtbl -> Reserved13(This) ) 

#define ITransactionProperty_Reserved14(This)	\
    ( (This)->lpVtbl -> Reserved14(This) ) 

#define ITransactionProperty_Reserved15(This)	\
    ( (This)->lpVtbl -> Reserved15(This) ) 

#define ITransactionProperty_Reserved16(This)	\
    ( (This)->lpVtbl -> Reserved16(This) ) 

#define ITransactionProperty_Reserved17(This)	\
    ( (This)->lpVtbl -> Reserved17(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionProperty_INTERFACE_DEFINED__ */


#ifndef __IMTSActivity_INTERFACE_DEFINED__
#define __IMTSActivity_INTERFACE_DEFINED__

/* interface IMTSActivity */
/* [object][unique][uuid][local] */ 


EXTERN_C const IID IID_IMTSActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51372AF0-CAE7-11CF-BE81-00AA00A2FA25")
    IMTSActivity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCall( 
            /* [in] */ IMTSCall *pCall) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsyncCall( 
            /* [in] */ IMTSCall *pCall) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved1( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BindToCurrentThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnbindFromThread( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMTSActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMTSActivity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMTSActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMTSActivity * This);
        
        DECLSPEC_XFGVIRT(IMTSActivity, SynchronousCall)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCall )( 
            IMTSActivity * This,
            /* [in] */ IMTSCall *pCall);
        
        DECLSPEC_XFGVIRT(IMTSActivity, AsyncCall)
        HRESULT ( STDMETHODCALLTYPE *AsyncCall )( 
            IMTSActivity * This,
            /* [in] */ IMTSCall *pCall);
        
        DECLSPEC_XFGVIRT(IMTSActivity, Reserved1)
        void ( STDMETHODCALLTYPE *Reserved1 )( 
            IMTSActivity * This);
        
        DECLSPEC_XFGVIRT(IMTSActivity, BindToCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *BindToCurrentThread )( 
            IMTSActivity * This);
        
        DECLSPEC_XFGVIRT(IMTSActivity, UnbindFromThread)
        HRESULT ( STDMETHODCALLTYPE *UnbindFromThread )( 
            IMTSActivity * This);
        
        END_INTERFACE
    } IMTSActivityVtbl;

    interface IMTSActivity
    {
        CONST_VTBL struct IMTSActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMTSActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMTSActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMTSActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMTSActivity_SynchronousCall(This,pCall)	\
    ( (This)->lpVtbl -> SynchronousCall(This,pCall) ) 

#define IMTSActivity_AsyncCall(This,pCall)	\
    ( (This)->lpVtbl -> AsyncCall(This,pCall) ) 

#define IMTSActivity_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define IMTSActivity_BindToCurrentThread(This)	\
    ( (This)->lpVtbl -> BindToCurrentThread(This) ) 

#define IMTSActivity_UnbindFromThread(This)	\
    ( (This)->lpVtbl -> UnbindFromThread(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMTSActivity_INTERFACE_DEFINED__ */



#ifndef __COMSVCSLib_LIBRARY_DEFINED__
#define __COMSVCSLib_LIBRARY_DEFINED__

/* library COMSVCSLib */
/* [helpstring][version][uuid] */ 



typedef /* [public][helpcontext][helpstring] */ 
enum __MIDL___MIDL_itf_autosvcs_0001_0150_0001
    {
        mtsErrCtxAborted	= 0x8004e002,
        mtsErrCtxAborting	= 0x8004e003,
        mtsErrCtxNoContext	= 0x8004e004,
        mtsErrCtxNotRegistered	= 0x8004e005,
        mtsErrCtxSynchTimeout	= 0x8004e006,
        mtsErrCtxOldReference	= 0x8004e007,
        mtsErrCtxRoleNotFound	= 0x8004e00c,
        mtsErrCtxNoSecurity	= 0x8004e00d,
        mtsErrCtxWrongThread	= 0x8004e00e,
        mtsErrCtxTMNotAvailable	= 0x8004e00f,
        comQCErrApplicationNotQueued	= 0x80110600,
        comQCErrNoQueueableInterfaces	= 0x80110601,
        comQCErrQueuingServiceNotAvailable	= 0x80110602,
        comQCErrQueueTransactMismatch	= 0x80110603,
        comqcErrRecorderMarshalled	= 0x80110604,
        comqcErrOutParam	= 0x80110605,
        comqcErrRecorderNotTrusted	= 0x80110606,
        comqcErrPSLoad	= 0x80110607,
        comqcErrMarshaledObjSameTxn	= 0x80110608,
        comqcErrInvalidMessage	= 0x80110650,
        comqcErrMsmqSidUnavailable	= 0x80110651,
        comqcErrWrongMsgExtension	= 0x80110652,
        comqcErrMsmqServiceUnavailable	= 0x80110653,
        comqcErrMsgNotAuthenticated	= 0x80110654,
        comqcErrMsmqConnectorUsed	= 0x80110655,
        comqcErrBadMarshaledObject	= 0x80110656
    } 	Error_Constants;


typedef /* [public] */ 
enum __MIDL___MIDL_itf_autosvcs_0001_0159_0001
    {
        LockSetGet	= 0,
        LockMethod	= ( LockSetGet + 1 ) 
    } 	LockModes;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_autosvcs_0001_0159_0002
    {
        Standard	= 0,
        Process	= ( Standard + 1 ) 
    } 	ReleaseModes;

#ifndef _tagCrmFlags_
#define _tagCrmFlags_
typedef 
enum tagCRMFLAGS
    {
        CRMFLAG_FORGETTARGET	= 0x1,
        CRMFLAG_WRITTENDURINGPREPARE	= 0x2,
        CRMFLAG_WRITTENDURINGCOMMIT	= 0x4,
        CRMFLAG_WRITTENDURINGABORT	= 0x8,
        CRMFLAG_WRITTENDURINGRECOVERY	= 0x10,
        CRMFLAG_WRITTENDURINGREPLAY	= 0x20,
        CRMFLAG_REPLAYINPROGRESS	= 0x40
    } 	CRMFLAGS;

#endif // _tagCrmFlags_
#ifndef _tagCrmRegFlags_
#define _tagCrmRegFlags_
typedef 
enum tagCRMREGFLAGS
    {
        CRMREGFLAG_PREPAREPHASE	= 0x1,
        CRMREGFLAG_COMMITPHASE	= 0x2,
        CRMREGFLAG_ABORTPHASE	= 0x4,
        CRMREGFLAG_ALLPHASES	= 0x7,
        CRMREGFLAG_FAILIFINDOUBTSREMAIN	= 0x10
    } 	CRMREGFLAGS;

#endif // _tagCrmRegFlags_

EXTERN_C const IID LIBID_COMSVCSLib;

EXTERN_C const CLSID CLSID_SecurityIdentity;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0a5-7f19-11d2-978e-0000f8757e2a")
SecurityIdentity;
#endif

EXTERN_C const CLSID CLSID_SecurityCallers;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0a6-7f19-11d2-978e-0000f8757e2a")
SecurityCallers;
#endif

EXTERN_C const CLSID CLSID_SecurityCallContext;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0a7-7f19-11d2-978e-0000f8757e2a")
SecurityCallContext;
#endif

EXTERN_C const CLSID CLSID_GetSecurityCallContextAppObject;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0a8-7f19-11d2-978e-0000f8757e2a")
GetSecurityCallContextAppObject;
#endif

EXTERN_C const CLSID CLSID_Dummy30040732;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0a9-7f19-11d2-978e-0000f8757e2a")
Dummy30040732;
#endif

EXTERN_C const CLSID CLSID_TransactionContext;

#ifdef __cplusplus

class DECLSPEC_UUID("7999FC25-D3C6-11CF-ACAB-00A024A55AEF")
TransactionContext;
#endif

EXTERN_C const CLSID CLSID_TransactionContextEx;

#ifdef __cplusplus

class DECLSPEC_UUID("5cb66670-d3d4-11cf-acab-00a024a55aef")
TransactionContextEx;
#endif

EXTERN_C const CLSID CLSID_ByotServerEx;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0aa-7f19-11d2-978e-0000f8757e2a")
ByotServerEx;
#endif

EXTERN_C const CLSID CLSID_CServiceConfig;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c8-7f19-11d2-978e-0000f8757e2a")
CServiceConfig;
#endif

EXTERN_C const CLSID CLSID_ServicePool;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c9-7f19-11d2-978e-0000f8757e2a")
ServicePool;
#endif

EXTERN_C const CLSID CLSID_ServicePoolConfig;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0ca-7f19-11d2-978e-0000f8757e2a")
ServicePoolConfig;
#endif

EXTERN_C const CLSID CLSID_SharedProperty;

#ifdef __cplusplus

class DECLSPEC_UUID("2A005C05-A5DE-11CF-9E66-00AA00A3F464")
SharedProperty;
#endif

EXTERN_C const CLSID CLSID_SharedPropertyGroup;

#ifdef __cplusplus

class DECLSPEC_UUID("2A005C0B-A5DE-11CF-9E66-00AA00A3F464")
SharedPropertyGroup;
#endif

EXTERN_C const CLSID CLSID_SharedPropertyGroupManager;

#ifdef __cplusplus

class DECLSPEC_UUID("2A005C11-A5DE-11CF-9E66-00AA00A3F464")
SharedPropertyGroupManager;
#endif

EXTERN_C const CLSID CLSID_COMEvents;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0ab-7f19-11d2-978e-0000f8757e2a")
COMEvents;
#endif

EXTERN_C const CLSID CLSID_CoMTSLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0ac-7f19-11d2-978e-0000f8757e2a")
CoMTSLocator;
#endif

EXTERN_C const CLSID CLSID_MtsGrp;

#ifdef __cplusplus

class DECLSPEC_UUID("4B2E958D-0393-11D1-B1AB-00AA00BA3258")
MtsGrp;
#endif

EXTERN_C const CLSID CLSID_ComServiceEvents;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c3-7f19-11d2-978e-0000f8757e2a")
ComServiceEvents;
#endif

EXTERN_C const CLSID CLSID_ComSystemAppEventData;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c6-7f19-11d2-978e-0000f8757e2a")
ComSystemAppEventData;
#endif

EXTERN_C const CLSID CLSID_CRMClerk;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0bd-7f19-11d2-978e-0000f8757e2a")
CRMClerk;
#endif

EXTERN_C const CLSID CLSID_CRMRecoveryClerk;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0be-7f19-11d2-978e-0000f8757e2a")
CRMRecoveryClerk;
#endif

EXTERN_C const CLSID CLSID_LBEvents;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c1-7f19-11d2-978e-0000f8757e2a")
LBEvents;
#endif

EXTERN_C const CLSID CLSID_MessageMover;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0bf-7f19-11d2-978e-0000f8757e2a")
MessageMover;
#endif

EXTERN_C const CLSID CLSID_DispenserManager;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabb0c0-7f19-11d2-978e-0000f8757e2a")
DispenserManager;
#endif

EXTERN_C const CLSID CLSID_PoolMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabafb5-7f19-11d2-978e-0000f8757e2a")
PoolMgr;
#endif

EXTERN_C const CLSID CLSID_EventServer;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabafbc-7f19-11d2-978e-0000f8757e2a")
EventServer;
#endif

EXTERN_C const CLSID CLSID_TrackerServer;

#ifdef __cplusplus

class DECLSPEC_UUID("ecabafb9-7f19-11d2-978e-0000f8757e2a")
TrackerServer;
#endif

EXTERN_C const CLSID CLSID_AppDomainHelper;

#ifdef __cplusplus

class DECLSPEC_UUID("ef24f689-14f8-4d92-b4af-d7b1f0e70fd4")
AppDomainHelper;
#endif

EXTERN_C const CLSID CLSID_ClrAssemblyLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("458aa3b5-265a-4b75-bc05-9bea4630cf18")
ClrAssemblyLocator;
#endif
#endif /* __COMSVCSLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_autosvcs_0000_0120 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0120_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_autosvcs_0000_0120_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [helpstring][local] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_GetApplicationProcessDetails_Proxy( 
    IGetAppTrackerData * This,
    /* [in] */ REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ DWORD Flags,
    /* [annotation] */ 
    _Out_opt_  ApplicationProcessSummary *Summary,
    /* [annotation] */ 
    _Out_opt_  ApplicationProcessStatistics *Statistics,
    /* [annotation] */ 
    _Out_opt_  ApplicationProcessRecycleInfo *RecycleInfo,
    /* [annotation] */ 
    _Out_opt_  BOOL *AnyComponentsHangMonitored);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_GetApplicationProcessDetails_Stub( 
    __RPC__in IGetAppTrackerData * This,
    /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ DWORD Flags,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessSummary *Summary,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessStatistics *Statistics,
    /* [unique][out][in] */ __RPC__inout_opt ApplicationProcessRecycleInfo *RecycleInfo,
    /* [unique][out][in] */ __RPC__inout_opt BOOL *AnyComponentsHangMonitored);

/* [helpstring][local] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_GetComponentDetails_Proxy( 
    IGetAppTrackerData * This,
    /* [in] */ REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ REFCLSID Clsid,
    /* [in] */ DWORD Flags,
    /* [annotation] */ 
    _Out_opt_  ComponentSummary *Summary,
    /* [annotation] */ 
    _Out_opt_  ComponentStatistics *Statistics,
    /* [annotation] */ 
    _Out_opt_  ComponentHangMonitorInfo *HangMonitorInfo);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IGetAppTrackerData_GetComponentDetails_Stub( 
    __RPC__in IGetAppTrackerData * This,
    /* [in] */ __RPC__in REFGUID ApplicationInstanceId,
    /* [in] */ DWORD ProcessId,
    /* [in] */ __RPC__in REFCLSID Clsid,
    /* [in] */ DWORD Flags,
    /* [unique][out][in] */ __RPC__inout_opt ComponentSummary *Summary,
    /* [unique][out][in] */ __RPC__inout_opt ComponentStatistics *Statistics,
    /* [unique][out][in] */ __RPC__inout_opt ComponentHangMonitorInfo *HangMonitorInfo);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


